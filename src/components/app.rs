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
use crate::state::ActiveMetadata;
use std::sync::{Arc, RwLock};
use std::fs::File;
use std::io::Read;
use gtk::*;

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

        window.set_titlebar (Some (&header.container));
        window.set_title ("eddit");
        window.set_wmclass ("text-editor", "eddit");
        window.set_default_size (800, 600);
        window.add (&content.container);

        window.show_all ();
        window.connect_delete_event(move |_, _| {
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

            self.open_file (current_file);
            //self.save_file ();
            //self.key_events ();
        }
        ConnectedApp (self)
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
}

impl ConnectedApp {
    pub fn execute (self) {
        self.0.window.show_all ();
        gtk::main ();
    }
}