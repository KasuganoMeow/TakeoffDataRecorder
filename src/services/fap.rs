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

use crate::utils::{directory, data, name};

pub fn fap_service_create(information: (String, String), directory: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let filename = name::gen_filename_with_name_and_now_time(
        information.0, 
        ".fap"
    );
    let path = directory::check_directory(&directory, &filename)?;
    data::create_data_file(&path)?;
    if let Err(error) = data::write_data_file(&path, &information.1) {
        if let Err(error) = fs::remove_file(&path) {
            return Err(Box::new(error));
        }
        return Err(error);
    };
    Ok(())
}
