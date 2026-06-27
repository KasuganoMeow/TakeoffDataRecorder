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

use rust_i18n::t;

use crate::utils::{self, directory};

pub fn get_database() -> Result<(Vec<(i64, String, String)>, usize, usize), Box<dyn std::error::Error>> {
    let dir = directory::get_database_directory();

    let mut pairs = Vec::new();
    let mut max_left = t!("database.collect.table_name").len();
    let mut max_right = t!("database.collect.table_time").len();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                
                if let Some((name, right_with_ext)) = name_str.split_once('-') {
                    let right = if let Some(dot_pos) = right_with_ext.rfind('.') {
                        &right_with_ext[..dot_pos]
                    } else {
                        right_with_ext
                    };
                    
                    if let Ok(timestamp) = right.parse::<i64>() {
                        let time_str = utils::time::unix_timestamp_to_utc(timestamp);
                            
                        max_left = max_left.max(name.len());
                        max_right = max_right.max(time_str.len());
                        pairs.push((timestamp, name.to_string(), time_str));
                    }
                }
            }
        }
    }

    pairs.sort_by_key(|(timestamp, _, _)| *timestamp);

    Ok((pairs, max_left, max_right))
}


