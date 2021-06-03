mod app;

use app::App;
use gdk::EventKey;
use gdk_pixbuf::Pixbuf;
use gtk::CellAreaExt;
use gtk::{false_, prelude::*};
use std::rc::Rc;
use std::{borrow::Borrow, error::Error, fs::File, io::Write, path::Path};
mod source;
use glib::clone;
fn download_image(url: &str, dst: &str) -> Result<(), Box<dyn Error>> {
    let response = reqwest::blocking::Client::new().get(url);
    let path = Path::new(dst);
    let mut file = File::create(&path)?;
    let content = response.send()?.bytes()?;
    file.write_all(&content[..])?;
    Ok(())
}

#[test]
fn test_download_image() {
    match download_image("https://bing.com/th?id=OHR.Wensleydale_ZH-CN8417818046_1920x1080.jpg&rf=LaDigue_1920x1080.jpg&pid=hp", "./test.jpg") {
        Ok(_)=>{}
        Err(err) => {
            print!("{}", err);
        }
    }
}

use std::process::Command;

fn set_wallpaper(script_path: &str, image_path: &str) -> Result<i32, std::io::Error> {
    let status = Command::new(script_path).arg(image_path).status()?;
    match status.code() {
        Some(code) => Ok(code),
        None => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "exit code is None",
        )),
    }
}

#[test]
fn test_set_wallpaper() {
    let status = set_wallpaper(
        "/home/nealian/desktop_new/wallpaper/shell/gsettings.sh",
        "/home/nealian/desktop_new/wallpaper/test.jpg",
    )
    .unwrap();
    assert_eq!(status, 0);
}

fn main1() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src =
        std::fs::read_to_string("/home/nealian/desktop_new/wallpaper/src/ui/window.ui").unwrap();
    // Then we call the Builder call.
    let builder = gtk::Builder::from_string(&glade_src);
    let window: gtk::Window = builder.get_object("window").unwrap();
    let search_bar: gtk::SearchBar = builder.get_object("search_bar").unwrap();
    let image_flow: gtk::FlowBox = builder.get_object("image_flow").unwrap();
    for _ in 0..18 {
        let image = gtk::Image::from_pixbuf(Some(
            &Pixbuf::from_file_at_scale(
                "/home/nealian/desktop_new/wallpaper/placeholder.jpg",
                192 * 2,
                108 * 2,
                false,
            )
            .unwrap(),
        ));
        image_flow.add(&image);
        // image_flow.get_child_at_index(0).unwrap().
    }
    window.set_resizable(false);
    window.show_all();
    let search_entry: gtk::SearchEntry = builder.get_object("search_entry").unwrap();
    // let search_bar = &search_bar;
    // search_entry.connect_key_press_event(move |x, e| {
    //     match e.get_keyval() {
    //         gdk::keys::constants::Return => {
    //             println!("do search");
    //         }
    //         gdk::keys::constants::Escape => {
    //             search_bar.hide();
    //         }
    //         _ => {}
    //     }
    //     return Inhibit(false);
    // });
    let btn_open_search: gtk::Button = builder.get_object("btn_open_search").unwrap();
    btn_open_search.connect_clicked(clone!(@weak search_bar => move |_| {
        if search_bar.get_visible() {
            search_bar.hide();
        } else {
            search_bar.show();
        }
    }));
    window.connect_destroy(|_| std::process::exit(0));
    // We start the gtk main loop.
    gtk::main();
}

fn main() {
    gtk::init().expect("Error initializing gtk.");
    App::run();
}
