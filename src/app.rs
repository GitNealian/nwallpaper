use gio::prelude::*;
use glib::clone;
use glib::{Receiver, Sender};
use gtk::{prelude::*, ApplicationWindow, Builder};
use std::{cell::RefCell, rc::Rc};

pub(crate) enum Action {
    SwitchOnline,
    SwitchLocal,
    SwitchColor,
}
#[derive(Debug)]
pub struct App {
    window: ApplicationWindow,
    image_flow: gtk::FlowBox,
    sender: Sender<Action>,
    receiver: RefCell<Option<Receiver<Action>>>,
}

impl App {
    fn new(application: &gtk::Application) -> Rc<Self> {
        let (sender, r) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let receiver = RefCell::new(Some(r));
        let glade_src = include_str!("ui/window.ui");
        let builder = Builder::from_string(glade_src);
        let window: ApplicationWindow = builder.get_object("window").unwrap();
        let image_flow: gtk::FlowBox = builder.get_object("image_flow").unwrap();
        window.set_application(Some(application));
        window.set_title("壁纸管理器");
        window.show_all();
        let app = App {
            window,
            image_flow,
            sender,
            receiver,
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
            });
        }));

        glib::set_application_name("nwallpaper");
        glib::set_prgname(Some("nwallpaper"));
        // gtk::Window::set_default_icon_name("nwallpaper");
        ApplicationExtManual::run(&application, &[]);
    }
}