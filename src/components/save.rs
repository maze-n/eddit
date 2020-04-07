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

use super::SaveDialog;
use super::misc::*;
use gtk::*;
use sourceview::*;
use crate::state::ActiveMetadata;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::sync::RwLock;

pub enum SaveAction {
    New (ActiveMetadata),
    Saved,
    Canceled,
}

pub fn save (editor: &Buffer, headerbar: &HeaderBar, save: &Button, current_file: &RwLock<Option<ActiveMetadata>>) {
    if let Some (text) = get_buffer (editor) {
        let result = write_data (current_file.read ().unwrap ().as_ref (), text.as_bytes ());

        match result {
            Ok (SaveAction::New(file)) => {
                set_title (headerbar, file.get_path ());
                if let Some (parent) = file.get_dir () {
                    let subtitle: &str = &parent.to_string_lossy ();
                    headerbar.set_subtitle (subtitle);
                }

                let mut current_file = current_file.write ().unwrap ();
                *current_file = Some (file);
                save.set_sensitive (false);
            }

            Ok (SaveAction::Saved) => {
                if let Some (ref mut current_file) = *current_file.write ().unwrap () {
                    current_file.set_sum (&text.as_bytes ());
                    save.set_sensitive (false);
                }
            }

            _ => (),
        }
    }
}

fn write_data (path: Option<&ActiveMetadata>, data: &[u8]) -> io::Result<SaveAction> {
    if let Some (path) = path {
        let mut file = OpenOptions::new ().create (true).write (true).truncate (true).open (path.get_path ())?;
        file.write_all (&data)?;
        return Ok(SaveAction::Saved);
    }

    let save_dialog = SaveDialog::new (None);
    if let Some (new_path) = save_dialog.run () {
        let mut file = OpenOptions::new ().create (true).write (true).truncate (true).open (&new_path)?;
        file.write_all (&data)?;
        Ok (SaveAction::New (ActiveMetadata::new (new_path, data)))
    } else {
        Ok (SaveAction::Canceled)
    }
}