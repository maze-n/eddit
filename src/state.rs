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

use std::path::{Path, PathBuf};
use tiny_keccak::keccak512;

pub struct ActiveMetadata {
    path: PathBuf,
    sum: [u8; 64],
}

impl ActiveMetadata {
    pub fn new(path: PathBuf, data: &[u8]) -> ActiveMetadata {
        ActiveMetadata {
            path,
            sum: keccak512(data),
        }
    }

    pub fn get_path<'a>(&'a self) -> &'a Path {
        &self.path
    }

    pub fn get_dir(&self) -> Option<PathBuf> {
        self.path.parent().map(|p| p.to_path_buf())
    }

    pub fn is_same_as(&self, data: &[u8]) -> bool {
        &keccak512(data)[..] == &self.sum[..]
    }

    pub fn set_sum(&mut self, data: &[u8]) {
        self.sum = keccak512(data);
    }
}
