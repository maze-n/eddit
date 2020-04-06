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
use sourceview::*;
use std::path::Path;

pub fn set_title (headerbar: &HeaderBar, path: &Path) {
    if let Some (filename) = path.file_name () {
        let filename: &str = &filename.to_string_lossy ();
        headerbar.set_title (filename);
    }
}

pub fn get_buffer (buffer: &Buffer) -> Option<String> {
    let start = buffer.get_start_iter ();
    let end = buffer.get_end_iter ();
    buffer.get_text (&start, &end, true)
}