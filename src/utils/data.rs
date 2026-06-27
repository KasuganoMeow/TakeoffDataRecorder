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
use std::path::PathBuf;

pub fn create_data_file(path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, "")?;
    Ok(())
}

pub fn read_data_file(path: &PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let contents: String = match fs::read_to_string(path) {
        Ok(contents) => contents,
        Err(error) => {
            return Err(Box::new(error));
        }
    };
    Ok(contents)
}

pub fn write_data_file(path: &PathBuf, content: &String) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, content)?;
    Ok(())
}
