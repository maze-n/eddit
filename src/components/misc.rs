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

use gio::SettingsExt;
use glib::GString;
use gtk::*;
use sourceview::*;
use std::path::Path;

pub fn set_title(headerbar: &HeaderBar, path: &Path) {
    if let Some(filename) = path.file_name() {
        let filename: &str = &filename.to_string_lossy();
        headerbar.set_title(Some(filename));
    }
}

pub fn get_buffer(buffer: &Buffer) -> Option<GString> {
    let (start, end) = buffer.get_bounds();
    buffer.get_text(&start, &end, true)
}

pub fn before_quit(window: &Window) {
    let size = window.get_size();
    let position = window.get_position();
    let settings = gio::Settings::new("com.github.maze-n.eddit");
    settings.set_int("pos-x", position.0);
    settings.set_int("pos-y", position.1);
    //FIXME: Only the first two set_int()'s are working .ie, only the pos-x and pos-y values are stored
    settings.set_int("window-width", size.0);
    settings.set_int("window-height", size.1);
}

pub fn set_sensitivity (entry: &SearchEntry, up: &Button, down: &Button, text: &str, iter: &TextIter) {
    let search_flag = TextSearchFlags::CASE_INSENSITIVE;
    if text != "" {
        if let Some(_) = iter.forward_search(text, search_flag, None) {
            down.set_sensitive(true);
        } else {
            down.set_sensitive(false);
        }
    
        if let Some(_) = iter.backward_search(text, search_flag, None) {
            up.set_sensitive(true);
        } else {
            up.set_sensitive(false);
        }
    } else {
        up.set_sensitive(false);
        down.set_sensitive(false);
    }

    if !up.get_sensitive() && !down.get_sensitive() && text != "" {
        entry.get_style_context().add_class(&gtk::STYLE_CLASS_ERROR);
        entry.set_icon_from_icon_name(EntryIconPosition::Primary, Some("dialog-error-symbolic"));
    } else {
        entry.get_style_context().remove_class(&gtk::STYLE_CLASS_ERROR);
        entry.set_icon_from_icon_name(EntryIconPosition::Primary, Some("edit-find-symbolic"));
    }
}