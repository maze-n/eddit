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

use gio::SettingsExt;
use gtk::*;
use pango::*;
use sourceview::*;

pub struct Content {
    pub container: ScrolledWindow,
    pub view: View,
    pub buff: Buffer,
    pub search_settings: SearchSettings,
    pub search_context: SearchContext,
}

impl Content {
    pub fn new() -> Content {
        let container = ScrolledWindow::new(NONE_ADJUSTMENT, NONE_ADJUSTMENT);
        let buff = Buffer::new(Some(&TextTagTable::new()));
        let view = View::new_with_buffer(&buff);
        let search_settings = SearchSettings::new();
        let search_context = SearchContext::new(&buff, Some(&search_settings));
        buff.place_cursor(&buff.get_start_iter());
        buff.set_highlight_matching_brackets(false);

        let settings = gio::Settings::new("com.github.maze-n.eddit");
        if let Some(font) = settings.get_string("font") {
            config_sourceview(&view, font.as_str().to_string());
        }

        container.add(&view);

        Content {
            container,
            buff,
            view,
            search_settings,
            search_context,
        }
    }
}

fn config_sourceview(view: &View, font: String) {
    WidgetExt::override_font(view, &FontDescription::from_string(font.as_str()));
    view.set_show_line_numbers(true);
    view.set_monospace(true);
    view.set_indent_width(4);
    view.set_smart_backspace(true);
    view.set_right_margin(10);
    view.set_left_margin(10);
}
