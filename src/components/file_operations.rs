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

use super::misc::*;
use super::OpenDialog;
use super::SaveDialog;
use crate::state::ActiveMetadata;
use gtk::*;
use sourceview::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::sync::RwLock;

pub enum SaveAction {
    New(ActiveMetadata),
    Saved,
    Canceled,
}

pub fn save(
    editor: &Buffer,
    headerbar: &HeaderBar,
    save: &Button,
    current_file: &RwLock<Option<ActiveMetadata>>,
) {
    if let Some(text) = get_buffer(editor) {
        let result = write_data(current_file.read().unwrap().as_ref(), text.as_bytes());

        match result {
            Ok(SaveAction::New(file)) => {
                set_title(headerbar, file.get_path());
                if let Some(parent) = file.get_dir() {
                    let subtitle: &str = &parent.to_string_lossy();
                    headerbar.set_subtitle(Some(subtitle));
                }

                let mut current_file = current_file.write().unwrap();
                *current_file = Some(file);
                save.set_sensitive(false);
            }

            Ok(SaveAction::Saved) => {
                if let Some(ref mut current_file) = *current_file.write().unwrap() {
                    current_file.set_sum(&text.as_bytes());
                    save.set_sensitive(false);
                }
            }

            _ => (),
        }
    }
}

fn write_data(path: Option<&ActiveMetadata>, data: &[u8]) -> io::Result<SaveAction> {
    if let Some(path) = path {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path.get_path())?;
        file.write_all(&data)?;
        return Ok(SaveAction::Saved);
    }

    let save_dialog = SaveDialog::new(None);
    if let Some(new_path) = save_dialog.run() {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(&new_path)?;
        file.write_all(&data)?;
        Ok(SaveAction::New(ActiveMetadata::new(new_path, data)))
    } else {
        Ok(SaveAction::Canceled)
    }
}

pub fn open(editor: &Buffer, headerbar: &HeaderBar, current_file: &RwLock<Option<ActiveMetadata>>) {
    let open_dialog = OpenDialog::new({
        let lock = current_file.read().unwrap();
        if let Some(ref path) = *lock {
            path.get_dir()
        } else {
            None
        }
    });

    if let Some(new_file) = open_dialog.run() {
        if let Ok(mut file) = File::open(&new_file) {
            let mut contents = String::new();
            let _ = file.read_to_string(&mut contents);

            set_title(&headerbar, &new_file);
            if let Some(parent) = new_file.parent() {
                let subtitle: &str = &parent.to_string_lossy();
                headerbar.set_subtitle(Some(subtitle));
            }

            *current_file.write().unwrap() =
                Some(ActiveMetadata::new(new_file, &contents.as_bytes()));
            editor.set_text(&contents);
            editor.place_cursor(&editor.get_start_iter());
        }
    }
}

pub fn open_from_files(
    editor: &Buffer,
    headerbar: &HeaderBar,
    current_file: &RwLock<Option<ActiveMetadata>>,
    path: String,
) {
    let new_file = PathBuf::from(path);
    if let Ok(mut file) = File::open(&new_file) {
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);

        set_title(&headerbar, &new_file);
        if let Some(parent) = new_file.parent() {
            let subtitle: &str = &parent.to_string_lossy();
            headerbar.set_subtitle(Some(subtitle));
        }

        *current_file.write().unwrap() = Some(ActiveMetadata::new(new_file, &contents.as_bytes()));
        editor.set_text(&contents);
        editor.place_cursor(&editor.get_start_iter());
    }
}
