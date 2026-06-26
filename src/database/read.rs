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

use std::{
    fs, 
    path::PathBuf, 
    process::exit
};

use crate::common::error::put_error;

pub fn read_fap_file(path: &PathBuf) -> String {
    let yaml = match fs::read_to_string(path) {
        Ok(yaml) => yaml,
        Err(error) => {
            put_error(error.to_string());
            exit(0);
        }
    };
    yaml
}
