use std::{error::Error, fs::File, io::Write, path::Path};

use gtk::prelude::*;
mod source;

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
        "shell/gsettings.sh",
        "file:///home/nealian/desktop_new/wallpaper/test.jpg",
    )
    .unwrap();
    assert_eq!(status, 0);
}

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }
    let glade_src = include_str!("ui/window.ui");
    // Then we call the Builder call.
    let builder = gtk::Builder::from_string(glade_src);
    let window: gtk::Window = builder.get_object("window").unwrap();
    // for i in 1..=9 {
    //     let image: gtk::Image = builder.get_object(&format!("image{}", i)).unwrap();
    //     if let Ok(pixbuf) = gdk_pixbuf::Pixbuf::from_file_at_scale(
    //         "/home/nealian/图片/30000000000243828_1920x1080.jpg",
    //         192,
    //         108,
    //         false,
    //     ) {
    //         image.set_from_pixbuf(Some(&pixbuf));
    //     }
    // }

    window.show_all();
    window.connect_destroy(|_| std::process::exit(0));
    // We start the gtk main loop.
    gtk::main();
}
