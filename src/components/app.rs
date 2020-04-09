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

use super::{Header, Content, OpenDialog};
use super::misc::*;
use super::save::*;
use crate::state::ActiveMetadata;
use std::sync::{Arc, RwLock};
use std::fs::File;
use std::io::Read;
use gtk::*;
use gio::{SettingsExt};

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

        window.set_titlebar (Some (&header.container));
        window.set_title ("eddit");
        window.set_wmclass ("text-editor", "eddit");
        window.set_default_size (800, 600);
        window.add (&content.container);

        window.connect_delete_event(move |window, _| {
            //let size = window.get_size ();
            let position = window.get_position ();
            settings.set_int ("pos-x", position.0);
            settings.set_int ("pos-y", position.1);
            //FIXME: For some reason only the first two set_int()'s seems to be working
            //settings.set_int ("window-width", size.0);
            //settings.set_int ("window-height", size.1);
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

            self.editor_changed (current_file.clone (), &self.header.save.clone ());
            self.open_file (current_file.clone ());
            self.save_file (&save.clone (), &save, current_file);
            //self.key_events ();
        }
        ConnectedApp (self)
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

        self.header.open.connect_clicked (move |_| {
            let open_dialog = OpenDialog::new ({
                let lock = current_file.read ().unwrap ();
                if let Some(ref path) = *lock {
                    path.get_dir ()
                } else {
                    None
                }
            });

            if let Some (new_file) = open_dialog.run () {
                if let Ok (mut file) = File::open (&new_file) {
                    let mut contents = String::new ();
                    let _ = file.read_to_string (&mut contents);
                    
                    set_title (&headerbar, &new_file);
                    if let Some (parent) = new_file.parent () {
                        let subtitle: &str = &parent.to_string_lossy ();
                        headerbar.set_subtitle (subtitle);
                    }

                    *current_file.write ().unwrap () = Some (ActiveMetadata::new (new_file, &contents.as_bytes ()));
                    editor.set_text (&contents);
                }
            }
        });
    }

    fn save_file (&self, save_button: &Button, actual_button: &Button, current_file: Arc<RwLock<Option<ActiveMetadata>>>) {
        let editor = self.content.buff.clone ();
        let headerbar = self.header.container.clone ();
        let save_button = save_button.clone ();
        actual_button.connect_clicked (
            move |_| save (&editor, &headerbar, &save_button, &current_file),
        );
    }
}

impl ConnectedApp {
    pub fn execute (self) {
        self.0.window.show_all ();
        gtk::main ();
    }
}