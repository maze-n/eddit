use gtk::*;
use pango::*;
use sourceview::*;

pub struct Content {
    pub container: ScrolledWindow,
    pub view: View,
    pub buff: Buffer,
}

impl Content {
    pub fn new () -> Content {
        let container = ScrolledWindow::new (None, None);
        let buff = Buffer::new (None);
        let view = View::new_with_buffer (&buff);

        config_sourceview (&view);

        container.add (&view);

        Content {
            container,
            buff,
            view,
        }
    }
}

fn config_sourceview (view: &View) {
    WidgetExt::override_font (view, &FontDescription::from_string ("monospace 12"));
    view.set_show_line_numbers(true);
    view.set_monospace(true);
    view.set_indent_width(4);
    view.set_smart_backspace(true);
    view.set_right_margin(10);
    view.set_left_margin(10);
}