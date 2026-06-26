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

use chrono::{
    DateTime, 
    Local
};
use rust_i18n::t;

pub fn get_database() -> (Vec<(i64, String, String)>, usize, usize) {
    let home = home::home_dir().unwrap_or_else(|| {
        eprintln!("{}", rust_i18n::t!("database.record.error_read_home"));
        std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
    });
    let database = home.join("TakeoffDataRecorder");

    let mut pairs = Vec::new();
    let mut max_left = t!("database.collect.table_name").len();
    let mut max_right = t!("database.collect.table_time").len();

    println!(" {}\n {}:\n", t!("database.collect.found_file"), database.to_string_lossy());

    for entry in match fs::read_dir(database) {
        Ok(read_dir) => read_dir,
        Err(error) => {
            crate::common::error::io_error(error);
            unreachable!();
        },
    } {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                crate::common::error::io_error(error);
                unreachable!();
            },
        };
        let path = entry.path();

        if path.is_file() {
            if let Some(name) = path.file_name() {
                let name_str = name.to_string_lossy();
                
                if let Some((left, right_with_ext)) = name_str.split_once('-') {
                    let right = if let Some(dot_pos) = right_with_ext.rfind('.') {
                        &right_with_ext[..dot_pos]
                    } else {
                        right_with_ext
                    };
                    
                    if let Ok(timestamp) = right.parse::<i64>() {
                        if let Some(utc) = DateTime::from_timestamp(timestamp, 0) {
                            let local: DateTime<Local> = utc.with_timezone(&Local);
                            let time_str = local.format("%Y-%m-%d %H:%M:%S %Z").to_string();
                            
                            max_left = max_left.max(left.len());
                            max_right = max_right.max(time_str.len());
                            pairs.push((timestamp, left.to_string(), time_str));
                        }
                    }
                }
            }
        }
    }

    pairs.sort_by_key(|(timestamp, _, _)| *timestamp);

    (pairs, max_left, max_right)
}

pub fn output_database(pairs: &[(i64, String, String)], max_left: usize, max_right: usize) {
    println!(
        " {:<width_left$} | {:<width_right$}", 
        t!("database.collect.table_name"), t!("database.collect.table_time"), 
        width_left = max_left - 2, 
        width_right = max_right
    );
    println!(
        " {:-<width_left$}-+-{:-<width_right$}", 
        "", "", 
        width_left = max_left, 
        width_right = max_right
    );
    
    for (_, left, right) in pairs {
        println!(
            " {:<width_left$} | {:<width_right$}", 
            left, right,
            width_left = max_left, 
            width_right = max_right
        );
    }
}
