use super::{Header, Content};

use gtk::*;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

impl App {
    pub fn new () -> App {
        if gtk::init ().is_err () {
            eprintln! ("Failed to initialize GTK");
            std::process::exit (1);
        }

        let window = Window::new (WindowType::Toplevel);
        let header = Header::new ();
        let content = Content::new ();

        window.set_titlebar (Some (&header.container));
        window.set_title ("eddit");
        window.set_wmclass ("text-editor", "eddit");
        window.set_default_size (800, 600);
        window.add (&content.container);

        window.show_all ();
        window.connect_delete_event(move |_, _| {
            main_quit();
            Inhibit(false)
        });
        gtk::main ();

        App {
            window,
            header,
            content,
        }
    }
}