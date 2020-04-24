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

pub struct SearchBox {
    pub container: Box,
    pub search_entry: SearchEntry,
    pub replace_entry: Entry,
    pub replace_button: Button,
    pub replace_all_button: Button,
    pub up: Button,
    pub down: Button,
    //pub case_sens_button: ToggleButton,
}

impl SearchBox {
    pub fn new() -> SearchBox {
        let container = Box::new(Orientation::Horizontal, 0);
        container.get_style_context().add_class("search-bar");

        let search_grid = Grid::new();
        search_grid
            .get_style_context()
            .add_class(&STYLE_CLASS_LINKED);
        search_grid.set_border_width(4);
        let search_entry = SearchEntry::new();
        search_entry.set_hexpand(true);
        search_entry.set_placeholder_text(Some("Find"));
        let up = Button::new_from_icon_name(Some("go-up-symbolic"), IconSize::SmallToolbar);
        up.set_tooltip_text(Some("Previous Item"));
        let down = Button::new_from_icon_name(Some("go-down-symbolic"), IconSize::SmallToolbar);
        down.set_tooltip_text(Some("Next Item"));
        //let case_sens_button = ToggleButton::new ();
        //case_sens_button.set_image (Some (&Image::new_from_icon_name (Some ("font-x-generic-symbolic"), IconSize::SmallToolbar)));
        //case_sens_button.set_tooltip_text (Some ("Set case sensitive searching"));

        //search_grid.add (&case_sens_button);
        search_grid.add(&search_entry);
        search_grid.add(&down);
        search_grid.add(&up);

        let replace_grid = Grid::new();
        replace_grid
            .get_style_context()
            .add_class(&STYLE_CLASS_LINKED);
        replace_grid.set_border_width(4);
        let replace_entry = Entry::new();
        replace_entry.set_hexpand(true);
        replace_entry.set_placeholder_text(Some("Replace With"));
        let replace_button = Button::new_with_label("Replace");
        let replace_all_button = Button::new_with_label("Replace All");

        replace_grid.add(&replace_entry);
        replace_grid.add(&replace_button);
        replace_grid.add(&replace_all_button);

        container.add(&search_grid);
        container.add(&replace_grid);

        SearchBox {
            container,
            search_entry,
            replace_entry,
            replace_button,
            replace_all_button,
            up,
            down,
            //case_sens_button,
        }
    }
}
