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

use super::{Header, Content};
use super::misc::*;
use super::file_operations::*;
use crate::state::ActiveMetadata;
use std::sync::{Arc, RwLock};
use std::env;
use gtk::*;
use gtk::SettingsExt as GTKSettingsExt;
use gio::{SettingsExt};
use pango::*;

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
}

pub struct ConnectedApp (App);

impl App {
    pub fn new () -> App {
        if gtk::init ().is_err () {
            eprintln! ("Failed to initialize GTK");
            std::process::exit (1);
        }

        let window = Window::new (WindowType::Toplevel);
        let header = Header::new ();
        let content = Content::new ();

        let settings = gio::Settings::new ("com.github.maze-n.eddit");
        let pos_x = settings.get_int ("pos-x");
        let pos_y = settings.get_int ("pos-y");
        let window_width = settings.get_int ("window-width");
        let window_height = settings.get_int ("window-height");
        if pos_x ==-1 && pos_y ==-1 {
            window.set_position (WindowPosition::Center);
        } else {
            window.move_ (pos_x, pos_y);
        }
        window.resize (window_width, window_height);
        if let Some (gtk_settings) = Settings::get_default () {
            let is_dark = settings.get_boolean ("is-dark");
            gtk_settings.set_property_gtk_application_prefer_dark_theme (is_dark);
        }

        window.set_titlebar (Some (&header.container));
        window.set_title ("eddit");
        window.set_default_size (800, 600);
        window.add (&content.container);

        let cloned_window = window.clone ();
        
        window.connect_delete_event(move |_, _| {
            before_quit (&cloned_window);
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            content,
        }
    }

    pub fn connect_events (self) -> ConnectedApp {
        let current_file = Arc::new (RwLock::new (None));
        {
            let save = &self.header.save;
            self.theme_changed (&self.header.theme_switch);
            self.editor_changed (current_file.clone (), &self.header.save.clone ());
            self.open_file (current_file.clone ());
            self.save_file (&save.clone (), &save, current_file.clone ());
            self.font_changed (&self.header.font_button);
            self.key_events (current_file);
        }
        ConnectedApp (self)
    }

    pub fn theme_changed (&self, theme_switch: &Switch) {
        let settings = gio::Settings::new ("com.github.maze-n.eddit");
        if let Some (gtk_settings) = Settings::get_default () {
            theme_switch.set_state (gtk_settings.get_property_gtk_application_prefer_dark_theme ());
        }
        theme_switch.connect_state_set (move |theme_switch, _| {
            if let Some (gtk_settings) = Settings::get_default () {
                gtk_settings.set_property_gtk_application_prefer_dark_theme (!theme_switch.get_state ());
                settings.set_boolean ("is-dark", !theme_switch.get_state ());
            }
            Inhibit (false)
        });
    }

    pub fn editor_changed (&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>, save_button: &Button) {
        let save_button = save_button.clone ();
        self.content.buff.connect_changed (move |editor| {
            if let Some (text) = get_buffer (&editor) {
                if let Some (ref current_file) = *current_file.read ().unwrap () {
                    let has_same_sum = current_file.is_same_as (&text.as_bytes ());
                    save_button.set_sensitive (!has_same_sum);
                }
            }
        });
    }

    fn open_file (&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
        let editor = self.content.buff.clone ();
        let headerbar = self.header.container.clone ();
        let args: Vec<String> = env::args ().collect ();
        if args.len () > 1 {
            open_from_files (&editor, &headerbar, &current_file, args [1].clone ());
        } 

        self.header.open.connect_clicked (
            move |_| open (&editor, &headerbar, &current_file),
        );
    }

    fn save_file (&self, save_button: &Button, actual_button: &Button, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
        let editor = self.content.buff.clone ();
        let headerbar = self.header.container.clone ();
        let save_button = save_button.clone ();
        actual_button.connect_clicked (
            move |_| save (&editor, &headerbar, &save_button, &current_file),
        );
    }

    fn font_changed (&self, actual_button: &FontButton) {
        let view = self.content.view.clone ();
        let font_button = actual_button.clone ();
        let settings = gio::Settings::new ("com.github.maze-n.eddit");
        actual_button.connect_font_set (move |_| {
            if let Some (fontname) = font_button.get_font_name () {
                WidgetExt::override_font (&view, &FontDescription::from_string (fontname.as_str ()));
                settings.set_string ("font", fontname.as_str ());
            }
        });
    }

    fn key_events (&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
        let editor = self.content.buff.clone ();
        let headerbar = self.header.container.clone ();
        let save_button = self.header.save.clone ();

        self.window.connect_key_press_event (move |_, gdk| {
            match gdk.get_keyval () {
                key if key == 's' as u32 && gdk.get_state ().contains (gdk::ModifierType::CONTROL_MASK) => {
                    save (&editor, &headerbar, &save_button, &current_file);
                },
                key if key == 'o' as u32 && gdk.get_state ().contains (gdk::ModifierType::CONTROL_MASK) => {
                    open (&editor, &headerbar, &current_file);
                }

                _ => (),
            }
            Inhibit (false)
        });
    }
}

impl ConnectedApp {
    pub fn execute (self) {
        self.0.window.show_all ();
        gtk::main ();
    }
}