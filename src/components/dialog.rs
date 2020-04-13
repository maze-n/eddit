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
use std::path::PathBuf;

pub struct OpenDialog (FileChooserDialog);

pub struct SaveDialog (FileChooserDialog);

impl OpenDialog {
    pub fn new (path: Option<PathBuf>) -> OpenDialog {
        let open_dialog = FileChooserDialog::new (
            Some ("Open"),
            Some (&Window::new (WindowType::Popup)),
            FileChooserAction::Open,
        );
        let filter = FileFilter::new ();
        filter.add_mime_type ("text/plain");
        filter.set_name (Some ("Text files"));
        open_dialog.add_filter (&filter);

        open_dialog.add_button ("Cancel", ResponseType::Cancel.into ());
        open_dialog.add_button ("Open", ResponseType::Ok.into ());

        path.map (|p| open_dialog.set_current_folder (p));
        OpenDialog (open_dialog)
    }

    pub fn run (&self) -> Option<PathBuf> {
        if self.0.run () == ResponseType::Ok.into () {
            self.0.get_filename ()
        } else {
            None
        }
    }
}

impl SaveDialog {
    pub fn new (path: Option<PathBuf>) -> SaveDialog {
        let save_dialog = FileChooserDialog::new (
            Some ("Save"),
            Some (&Window::new (WindowType::Popup)),
            FileChooserAction::Save,
        );

        save_dialog.add_button ("Cancel", ResponseType::Cancel.into ());
        save_dialog.add_button ("Save", ResponseType::Ok.into ());

        path.map (|p| save_dialog.set_current_folder (p));

        SaveDialog (save_dialog)
    }

    pub fn run (&self) -> Option<PathBuf> {
        if self.0.run () == ResponseType::Ok.into () {
            self.0.get_filename ()
        } else {
            None
        }
    }
}

impl Drop for OpenDialog {
    fn drop (&mut self) { self.0.destroy (); }
}

impl Drop for SaveDialog {
    fn drop (&mut self) { self.0.destroy (); }
}