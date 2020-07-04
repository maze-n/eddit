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

pub struct OpenDialog(FileChooserDialog);

pub struct SaveDialog(FileChooserDialog);

pub struct UnsavedDialog(Dialog);

pub struct ErrorDialog(Dialog);

impl OpenDialog {
    pub fn new(path: Option<PathBuf>) -> OpenDialog {
        let open_dialog = FileChooserDialog::new(
            Some("Open"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Open,
        );
        let filter = FileFilter::new();
        filter.add_mime_type("text/plain");
        filter.set_name(Some("Text files"));
        open_dialog.add_filter(&filter);

        open_dialog.add_button("Cancel", ResponseType::Cancel.into());
        open_dialog.add_button("Open", ResponseType::Ok.into());

        path.map(|p| open_dialog.set_current_folder(p));
        OpenDialog(open_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl SaveDialog {
    pub fn new(path: Option<PathBuf>) -> SaveDialog {
        let save_dialog = FileChooserDialog::new(
            Some("Save"),
            Some(&Window::new(WindowType::Popup)),
            FileChooserAction::Save,
        );

        save_dialog.add_button("Cancel", ResponseType::Cancel.into());
        save_dialog.add_button("Save", ResponseType::Ok.into());

        path.map(|p| save_dialog.set_current_folder(p));

        SaveDialog(save_dialog)
    }

    pub fn run(&self) -> Option<PathBuf> {
        if self.0.run() == ResponseType::Ok.into() {
            self.0.get_filename()
        } else {
            None
        }
    }
}

impl UnsavedDialog {
    pub fn new(window: &Window) -> UnsavedDialog {
        let unsaved_dialog = Dialog::new_with_buttons(
            Some("Warning"),
            Some(window),
            DialogFlags::DESTROY_WITH_PARENT,
            &[],
        );

        let dialog_box = unsaved_dialog.get_content_area();
        
        let dialog_grid = Box::new(Orientation::Horizontal, 20);
        dialog_grid.set_border_width(20);

        let warning_image = Image::new_from_icon_name(Some("dialog-warning"), IconSize::Dialog);
        let head_label = Label::new(Some("Save the file before closing?"));
        head_label.set_markup("<big><b>Save the file before closing?</b></big>");
        let sub_label = Label::new(Some("Your changes will be lost if you don't save them."));

        let label_box = Box::new(Orientation::Vertical, 4);
        label_box.add(&head_label);
        label_box.add(&sub_label);

        dialog_grid.add(&warning_image);
        dialog_grid.add(&label_box);

        dialog_box.add(&dialog_grid);

        let save_button = Button::new_with_label("Save");
        save_button.get_style_context().add_class("suggested-action");

        let unsave_button = Button::new_with_label("Don't Save");

        unsaved_dialog.add_action_widget(&unsave_button, ResponseType::No);
        unsaved_dialog.add_action_widget(&save_button, ResponseType::Yes);

        unsaved_dialog.set_resizable(false);
        unsaved_dialog.show_all();

        UnsavedDialog(unsaved_dialog)
    }

    pub fn run(&self) -> ResponseType {
        self.0.run()
    }
}

impl ErrorDialog {
    pub fn new(window: &Window) -> ErrorDialog {
        let error_dialog = Dialog::new_with_buttons(
            Some("Error"),
            Some(window),
            DialogFlags::DESTROY_WITH_PARENT,
            &[("OK", ResponseType::Ok)],
        );

        let dialog_box = error_dialog.get_content_area();
        
        let dialog_grid = Box::new(Orientation::Horizontal, 20);
        dialog_grid.set_border_width(20);

        let error_image = Image::new_from_icon_name(Some("dialog-error"), IconSize::Dialog);
        let head_label = Label::new(Some("Failed to save the file"));
        head_label.set_markup("<big><b>Failed to save the file</b></big>");
        let sub_label = Label::new(Some("The file may be read-only, or another user may have it open. Try saving the file\n with a different name or in a different folder."));
        let label_box = Box::new(Orientation::Vertical, 4);
        label_box.add(&head_label);
        label_box.add(&sub_label);

        dialog_grid.add(&error_image);
        dialog_grid.add(&label_box);
        dialog_box.add(&dialog_grid);

        error_dialog.set_resizable(false);
        error_dialog.show_all();

        ErrorDialog(error_dialog)
    }

    pub fn run(&self) -> ResponseType {
        self.0.run()
    }
}

impl Drop for OpenDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}

impl Drop for SaveDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}

impl Drop for UnsavedDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}

impl Drop for ErrorDialog {
    fn drop(&mut self) {
        self.0.destroy();
    }
}