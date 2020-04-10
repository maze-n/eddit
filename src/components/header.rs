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
        open.set_tooltip_text ("Open a file\n   Ctrl + O");
        let save = Button::new_from_icon_name ("document-save", 32);
        save.set_tooltip_text ("Save file\n Ctrl + S");

        container.pack_start (&open);
        container.pack_start (&save);

        Header {
            container,
            open,
            save,
        }
    }
}