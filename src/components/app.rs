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

use super::file_operations::*;
use super::misc::*;
use super::{Content, Header, SearchBox};
use crate::state::ActiveMetadata;
use gio::SettingsExt;
use gtk::SettingsExt as GTKSettingsExt;
use gtk::*;
use pango::*;
use sourceview::*;
use std::env;
use std::sync::{Arc, RwLock};

pub struct App {
    pub window: Window,
    pub header: Header,
    pub content: Content,
    pub search_bar: SearchBox,
    pub revealer: Revealer,
}

pub struct ConnectedApp(App);

impl App {
    pub fn new() -> App {
        if gtk::init().is_err() {
            eprintln!("Failed to initialize GTK");
            std::process::exit(1);
        }

        let window = Window::new(WindowType::Toplevel);
        let header = Header::new();
        let content = Content::new();

        let window_box = Box::new(Orientation::Vertical, 0);

        let revealer = Revealer::new();
        let search_bar = SearchBox::new();
        revealer.set_transition_type(RevealerTransitionType::SlideDown);
        revealer.add(&search_bar.container);

        let settings = gio::Settings::new("com.github.maze-n.eddit");
        let pos_x = settings.get_int("pos-x");
        let pos_y = settings.get_int("pos-y");
        settings.set_int("pos-x", pos_x + 20);
        settings.set_int("pos-y", pos_y + 20);
        let window_width = settings.get_int("window-width");
        let window_height = settings.get_int("window-height");
        if pos_x == -1 && pos_y == -1 {
            window.set_position(WindowPosition::Center);
        } else {
            window.move_(pos_x, pos_y);
        }
        window.resize(window_width, window_height);
        if let Some(gtk_settings) = Settings::get_default() {
            let is_dark = settings.get_boolean("is-dark");
            gtk_settings.set_property_gtk_application_prefer_dark_theme(is_dark);
        }

        window_box.pack_start(&revealer, false, true, 0);
        window_box.pack_start(&content.container, true, true, 0);

        window.set_titlebar(Some(&header.container));
        window.set_title("eddit");
        window.set_default_size(800, 600);
        window.add(&window_box);

        let cloned_window = window.clone();

        window.connect_delete_event(move |_, _| {
            before_quit(&cloned_window);
            main_quit();
            Inhibit(false)
        });

        App {
            window,
            header,
            content,
            search_bar,
            revealer,
        }
    }

    pub fn connect_events(self) -> ConnectedApp {
        let current_file = Arc::new(RwLock::new(None));
        {
            let save = &self.header.save;
            self.theme_changed(&self.header.theme_switch);
            self.editor_changed(current_file.clone(), &self.header.save.clone());
            self.open_file(current_file.clone());
            self.save_file(&save.clone(), &save, current_file.clone());
            self.font_changed(&self.header.font_button);
            self.find_replace(&self.header.find_button, &self.revealer, &self.search_bar.search_entry);
            self.key_events(current_file);
        }
        ConnectedApp(self)
    }

    pub fn find_replace(&self, find_button: &ToggleButton, revealer: &Revealer, search_entry: &SearchEntry) {
        let revealer = revealer.clone();
        //let case_sens_button = self.search_bar.case_sens_button.clone ();
        let search_entry = search_entry.clone();
        let up = self.search_bar.up.clone();
        let down = self.search_bar.down.clone();
        let replace_button = self.search_bar.replace_button.clone();
        let replace_all = self.search_bar.replace_all_button.clone();
        let replace_entry = self.search_bar.replace_entry.clone();
        let buff = self.content.buff.clone();
        let view = self.content.view.clone();

        let search_settings = self.content.search_settings.clone();
        let search_context = self.content.search_context.clone();
        let search_flag = TextSearchFlags::CASE_INSENSITIVE;

        let search_settings_clone = search_settings.clone();
        let entry = search_entry.clone();
        find_button.connect_toggled(move |find_button| {
            revealer.set_reveal_child(find_button.get_active());
            entry.grab_focus();
            search_settings_clone.set_search_text(Some(""));
        });

        //let search_settings_clone = search_settings.clone ();
        //let case_sens_clone = case_sens_button.clone ();
        //let search_flag_clone = search_flag.clone ();
        //case_sens_clone.connect_toggled (move |case_sens_clone| {
        //    search_settings_clone.set_case_sensitive (case_sens_clone.get_active ());
        //});

        let up_clone = up.clone();
        let down_clone = down.clone();
        let search_settings_clone = search_settings.clone();
        let buffer = buff.clone();
        let search_entry_clone = search_entry.clone();
        search_entry_clone.connect_search_changed(move |entry| {
            let iter = buffer.get_iter_at_offset(buffer.get_property_cursor_position());
            if let Some(text) = entry.get_text() {
                let text = text.as_str();
                search_settings_clone.set_search_text(Some(text));
                set_sensitivity(&entry, &up_clone, &down_clone, text, &iter);
            }
        });

        let up_clone = up.clone();
        let down_clone = down.clone();
        let buffer = buff.clone();
        let search_settings_clone = search_settings.clone();
        let view_clone = view.clone();
        let search_entry_clone = search_entry.clone();
        down.connect_clicked(move |down| {
            if let Some(iters) = buffer.get_selection_bounds() {
                let mut iter = iters.1;
                if let Some(search_string) = search_settings_clone.get_search_text() {
                    if let Some(mut match_iters) = iter.forward_search(search_string.as_str(), search_flag, None) {
                        buffer.select_range(&match_iters.0, &match_iters.1);
                        view_clone.scroll_to_iter(&mut match_iters.0, 0.0, false, 0.0, 0.0);
                        iter = match_iters.1;
                    }
                    set_sensitivity(&search_entry_clone, &up_clone, &down, search_string.as_str(), &iter);
                }
            } else {
                let mut iter = buffer.get_iter_at_offset(buffer.get_property_cursor_position());
                if let Some(search_string) = search_settings_clone.get_search_text() {
                    if let Some(mut match_iters) = iter.forward_search(search_string.as_str(), search_flag, None) {
                        buffer.select_range(&match_iters.0, &match_iters.1);
                        view_clone.scroll_to_iter(&mut match_iters.0, 0.0, false, 0.0, 0.0);
                        iter = match_iters.1;
                    }
                    set_sensitivity(&search_entry_clone, &up_clone, &down, search_string.as_str(), &iter);
                }
            }
        });

        let buffer = buff.clone();
        up.connect_clicked(move |up| {
            if let Some(iters) = buffer.get_selection_bounds() {
                let mut iter = iters.0;
                if let Some(search_string) = search_settings.get_search_text() {
                    if let Some(mut match_iters) = iter.backward_search(search_string.as_str(), search_flag, None) {
                        buffer.select_range(&match_iters.0, &match_iters.1);
                        view.scroll_to_iter(&mut match_iters.0, 0.0, false, 0.0, 0.0);
                        iter = match_iters.0;
                    }
                    set_sensitivity(&search_entry, &up, &down_clone, search_string.as_str(), &iter);
                }
            } else {
                let mut iter = buffer.get_iter_at_offset(buffer.get_property_cursor_position());
                if let Some(search_string) = search_settings.get_search_text() {
                    if let Some(mut match_iters) = iter.backward_search(search_string.as_str(), search_flag, None) {
                        buffer.select_range(&match_iters.0, &match_iters.1);
                        view.scroll_to_iter(&mut match_iters.0, 0.0, false, 0.0, 0.0);
                        iter = match_iters.0
                    }
                    set_sensitivity(&search_entry, &up, &down_clone, search_string.as_str(), &iter);
                }
            }
        });

        let replace_clone = replace_button.clone();
        let replace_all_clone = replace_all.clone();
        let buffer = buff.clone();
        replace_entry.connect_changed(move |replace_entry| {
            if let Some(text) = replace_entry.get_text() {
                if text.as_str() != "" {
                    if let Some(_) = buffer.get_selection_bounds() {
                        replace_clone.set_sensitive(true);
                        replace_all_clone.set_sensitive(true);
                    } else {
                        replace_clone.set_sensitive(false);
                        replace_all_clone.set_sensitive(false);
                    }
                } else {
                    replace_clone.set_sensitive(false);
                    replace_all_clone.set_sensitive(false);
                }
            }
        });

        let search_context_clone = search_context.clone();
        let replace_entry_clone = replace_entry.clone();
        let down_clone = down.clone();
        replace_button.connect_clicked(move |_| {
            if let Some(match_selected) = buff.get_selection_bounds() {
                if let Some(replace_text) = replace_entry_clone.get_text() {
                    search_context_clone.replace(&match_selected.0, &match_selected.1, replace_text.as_str());
                    down_clone.clicked();
                }
            }
        });

        replace_all.connect_clicked(move |_| {
            if let Some(text) = replace_entry.get_text() {
                search_context.replace_all(text.as_str());
            }
        });
    }

    pub fn theme_changed(&self, theme_switch: &Switch) {
        let settings = gio::Settings::new("com.github.maze-n.eddit");
        if let Some(gtk_settings) = Settings::get_default() {
            theme_switch.set_state(gtk_settings.get_property_gtk_application_prefer_dark_theme());
        }
        theme_switch.connect_state_set(move |theme_switch, _| {
            if let Some(gtk_settings) = Settings::get_default() {
                gtk_settings
                    .set_property_gtk_application_prefer_dark_theme(!theme_switch.get_state());
                settings.set_boolean("is-dark", !theme_switch.get_state());
            }
            Inhibit(false)
        });
    }

    pub fn editor_changed(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>, save_button: &Button) {
        let save_button = save_button.clone();
        self.content.buff.connect_changed(move |editor| {
            if let Some(text) = get_buffer(&editor) {
                if let Some(ref current_file) = *current_file.read().unwrap() {
                    let has_same_sum = current_file.is_same_as(&text.as_bytes());
                    save_button.set_sensitive(!has_same_sum);
                }
            }
        });
    }

    fn open_file(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
        let editor = self.content.buff.clone();
        let headerbar = self.header.container.clone();
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            open_from_files(&editor, &headerbar, &current_file, args[1].clone());
        }

        self.header
            .open
            .connect_clicked(move |_| open(&editor, &headerbar, &current_file));
    }

    fn save_file(
        &self,
        save_button: &Button,
        actual_button: &Button,
        current_file: Arc<RwLock<Option<ActiveMetadata>>>,
    ) {
        let editor = self.content.buff.clone();
        let headerbar = self.header.container.clone();
        let save_button = save_button.clone();
        actual_button
            .connect_clicked(move |_| save(&editor, &headerbar, &save_button, &current_file));
    }

    fn font_changed(&self, actual_button: &FontButton) {
        let view = self.content.view.clone();
        let font_button = actual_button.clone();
        let settings = gio::Settings::new("com.github.maze-n.eddit");
        actual_button.connect_font_set(move |_| {
            if let Some(fontname) = font_button.get_font_name() {
                WidgetExt::override_font(&view, &FontDescription::from_string(fontname.as_str()));
                settings.set_string("font", fontname.as_str());
            }
        });
    }

    fn key_events(&self, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
        let editor = self.content.buff.clone();
        let headerbar = self.header.container.clone();
        let save_button = self.header.save.clone();
        let find_button = self.header.find_button.clone();
        let search_entry = self.search_bar.search_entry.clone();

        self.window.connect_key_press_event(move |_, gdk| {
            match gdk.get_keyval() {
                key if key == 's' as u32
                    && gdk.get_state().contains(gdk::ModifierType::CONTROL_MASK) =>
                {
                    save(&editor, &headerbar, &save_button, &current_file);
                }
                key if key == 'o' as u32
                    && gdk.get_state().contains(gdk::ModifierType::CONTROL_MASK) =>
                {
                    open(&editor, &headerbar, &current_file);
                }
                key if key == 'f' as u32
                    && gdk.get_state().contains(gdk::ModifierType::CONTROL_MASK) =>
                {
                    find_button.set_active(true);
                    search_entry.grab_focus();
                }
                key if key == gdk::enums::key::Escape => {
                    find_button.set_active(false);
                }

                _ => (),
            }
            Inhibit(false)
        });
    }
}

impl ConnectedApp {
    pub fn execute(self) {
        self.0.window.show_all();
        gtk::main();
    }
}
