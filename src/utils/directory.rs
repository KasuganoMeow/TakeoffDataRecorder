/* SPDX-License-Identifier: AGPL-3.0-or-later */
/* Copyright 2026 KasuganoMeow */
/*
This file is part of TakeoffDataRecorder (ToDR).

ToDR is free software: you can redistribute it and/or modify it under the terms of
the GNU Affero General Public License as published by the Free Software Foundation, 
either version 3 of the License, or (at your option) any later version.

ToDR is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY;
without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
PURPOSE. See the GNU Affero General Public License for more details.

You should have received a copy of the GNU Affero General Public License along with 
ToDR. If not, see <https://www.gnu.org/licenses/>.
*/

use std::fs;
use std::path::{Path, PathBuf};

use rust_i18n::t;

pub fn check_directory(directory: &PathBuf, filename: &String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path: PathBuf = {
        if directory.exists() || fs::create_dir_all(&directory).is_ok() {
            Path::new(&directory).join(&filename)
        } else {
            Path::new(".").join(&filename)
        }
    };
    Ok(path)
}

pub fn get_database_directory() -> PathBuf {
    let home = home::home_dir().unwrap_or_else(|| {
        eprintln!(" {}", t!("utils.directory.error_get_home"));
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });
    home.join("TakeoffDataRecorder")
}
