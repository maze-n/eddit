use gtk::*;

pub struct Header {
    pub container: HeaderBar,
    pub open: Button,
    pub save: Button,
}

impl Header {
    pub fn new () -> Header {
        let container = HeaderBar::new ();
        container.set_title ("eddit");
        container.set_show_close_button (true);

        let open = Button::new_from_icon_name ("document-open", 32);
        let save = Button::new_from_icon_name ("document-save", 32);

        container.pack_start (&open);
        container.pack_start (&save);

        Header {
            container,
            open,
            save,
        }
    }
}