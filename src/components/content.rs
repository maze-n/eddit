/*
 * Copyright (c) 2020 mazen (mmaz999@outlook.com)
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Library General Public License as published by
 * the Free Software Foundation, either version 2.1 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Library General Public License for more details.
 *
 * You should have received a copy of the GNU Library General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 *
 * Authored by: mazen <https://github.com/maze-n>
 */

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