use super::source::Source;
use gdk_pixbuf::Pixbuf;
use gio::prelude::*;
use glib::clone;
use glib::{Receiver, Sender};
use gtk::{ApplicationWindow, Builder, Image, prelude::*};
use std::convert::TryInto;
use std::thread;
use std::time::Duration;
use std::{cell::RefCell, rc::Rc};
pub(crate) enum Action {
    SwitchOnline,
    SwitchLocal,
    SwitchColor,
    ShowImage(String, String, i32),
    ShowImagePlaceHolder(i32),
}
#[derive(Debug)]
pub struct App {
    window: ApplicationWindow,
    image_grid: gtk::Grid,
    sender: Sender<Action>,
    receiver: RefCell<Option<Receiver<Action>>>,
    source: Source,
}

impl App {
    fn new(application: &gtk::Application) -> Rc<Self> {
        let (sender, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let receiver = RefCell::new(Some(r));
        let glade_src = include_str!("ui/window.ui");
        let builder = Builder::from_string(glade_src);
        let window: ApplicationWindow = builder.get_object("window").unwrap();
        let image_grid: gtk::Grid = builder.get_object("image_grid").unwrap();
        window.set_application(Some(application));
        window.set_title("壁纸管理器");
        window.show_all();
        let sender_new = sender.clone();
        let app = App {
            window,
            image_grid,
            sender,
            receiver,
            source: Source::new(sender_new),
        };
        Rc::new(app)
    }
    fn init(app: &Rc<Self>) {
        let app = Rc::clone(app);
        let receiver = app.receiver.borrow_mut().take().unwrap();
        receiver.attach(None, move |action| app.do_action(action));
    }

    fn do_action(&self, action: Action) -> glib::Continue {
        match action {
            Action::SwitchOnline => {}
            Action::SwitchLocal => {}
            Action::SwitchColor => {}
            Action::ShowImage(path, name, index) => {
                let child = self.image_grid.get_child_at(index % 3, index / 3).unwrap();
                self.image_grid.remove(&child);
                let image = gtk::Image::from_pixbuf(Some(
                    &Pixbuf::from_file_at_scale(
                        &format!("/home/nealian/desktop_new/wallpaper/{}", path),
                        192 * 2,
                        108 * 2,
                        false,
                    )
                    .unwrap(),
                ));
                self.image_grid.attach(&image, index % 3, index / 3, 1, 1);
                self.image_grid.show_all();
                println!("{}, {}, {}\n", path, name, index);
            }
            Action::ShowImagePlaceHolder(num) => {
                // clear all images
                for i in 0..9 {
                    if let Some(child) = self.image_grid.get_child_at(i % 3, i / 3){
                        self.image_grid.remove(&child);
                    }
                }
                for i in 0..num {
                    let image = gtk::Image::from_pixbuf(Some(
                        &Pixbuf::from_file_at_scale(
                            "/home/nealian/desktop_new/wallpaper/placeholder.jpg",
                            192 * 2,
                            108 * 2,
                            false,
                        )
                        .unwrap(),
                    ));
                    self.image_grid.attach(&image, i % 3, i / 3, 1, 1);
                }
                self.image_grid.show_all();
            }
        }
        glib::Continue(true)
    }

    pub fn run() {
        let application = gtk::Application::new(Some("cn.nealian.nwallpaper"), Default::default())
            .expect("Application initialization failed...");

        application.connect_startup(clone!(@weak application => move |_| {
            let app = Self::new(&application);
            Self::init(&app);
            application.connect_activate(move |_| {
                app.window.show_now();
                app.window.present();
                app.sender.send(Action::ShowImagePlaceHolder(9)).unwrap();
                let source = app.source.clone();
                thread::spawn(move ||{
                    thread::sleep(Duration::from_secs(10));
                    source.get_image("","".to_owned(),0,9,0,0).unwrap();
                });
            });
        }));

        glib::set_application_name("nwallpaper");
        glib::set_prgname(Some("nwallpaper"));
        gtk::Window::set_default_icon_name("applications-graphics");
        ApplicationExtManual::run(&application, &[]);
    }
}
