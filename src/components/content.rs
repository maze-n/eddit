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
use gio::{SettingsExt};
use pango::*;
use sourceview::*;

pub struct Content {
    pub container: ScrolledWindow,
    pub view: View,
    pub buff: TextBuffer,
}

impl Content {
    pub fn new () -> Content {
        let container = ScrolledWindow::new (NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        //let buff = Buffer::new (None);
        let view = View::new ();
        let buff = view.get_buffer ().unwrap ();

        let settings = gio::Settings::new ("com.github.maze-n.eddit");
        if let Some (font) = settings.get_string ("font"){
            config_sourceview (&view, font.as_str ().to_string ());
        }

        container.add (&view);

        Content {
            container,
            buff,
            view,
        }
    }
}

fn config_sourceview (view: &View, font: String) {
    WidgetExt::override_font (view, &FontDescription::from_string (font.as_str ()));
    view.set_show_line_numbers(true);
    view.set_monospace(true);
    view.set_indent_width(4);
    view.set_smart_backspace(true);
    view.set_right_margin(10);
    view.set_left_margin(10);
}