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

pub struct Header {
    pub container: HeaderBar,
    pub open: Button,
    pub save: Button,
    //pub theme_switch: Switch,
    pub font_button: FontButton,
}

impl Header {
    pub fn new () -> Header {
        let container = HeaderBar::new ();
        container.set_title (Some ("eddit"));
        container.set_show_close_button (true);

        let settings = gio::Settings::new ("com.github.maze-n.eddit");

        let open = Button::new_from_icon_name (Some ("document-open"), IconSize::LargeToolbar);
        open.set_tooltip_text (Some ("Open a file\n   Ctrl + O"));
        open.set_valign (Align::Center);
        let save = Button::new_from_icon_name (Some ("document-save"), IconSize::LargeToolbar);
        save.set_tooltip_text (Some ("Save file\n Ctrl + S"));
        save.set_valign (Align::Center);
        let menu_button = MenuButton::new ();
        menu_button.set_tooltip_text (Some ("Preferences"));
        menu_button.set_valign (Align::Center);
        menu_button.set_image (Some(&Image::new_from_icon_name (Some ("open-menu"), IconSize::LargeToolbar)));

        let popover = Popover::new (Some (&menu_button));
        let pop_container = Box::new (Orientation::Vertical, 6);
        pop_container.set_border_width (12);

        let theme_selector = Grid::new ();
        theme_selector.set_column_homogeneous (true);
        theme_selector.set_row_homogeneous (true);
        theme_selector.set_column_spacing (12);
        theme_selector.set_hexpand (true);

        let light_icon = Image::new_from_icon_name (Some ("brightness-display-symbolic"), IconSize::LargeToolbar);
        light_icon.set_halign (Align::End);
        let dark_icon = Image::new_from_icon_name (Some ("weather-clear-night-symbolic"), IconSize::LargeToolbar);
        dark_icon.set_halign (Align::Start);
        let theme_switch = Switch::new ();
        theme_switch.set_tooltip_text (Some ("Toggle light/dark theme"));
        theme_switch.set_halign (Align::Center);

        theme_selector.add (&light_icon);
        theme_selector.add (&theme_switch);
        theme_selector.add (&dark_icon);

        let font_button = FontButton::new ();
        font_button.set_halign (Align::Center);
        font_button.set_tooltip_text (Some ("Select font and size"));
        font_button.set_use_font (true);
        if let Some (font) = settings.get_string ("font"){
            font_button.set_font (font.as_str ());
        }

        //pop_container.pack_start (&theme_selector, true, true, 0);
        //pop_container.pack_start (&Separator::new (Orientation::Horizontal), true, true, 6);
        pop_container.pack_start (&font_button, true, true, 0);
        pop_container.show_all ();

        popover.add (&pop_container);

        menu_button.set_popover (Some (&popover));

        container.pack_start (&open);
        container.pack_start (&save);
        container.pack_end (&menu_button);

        Header {
            container,
            open,
            save,
            //theme_switch,
            font_button,
        }
    }
}