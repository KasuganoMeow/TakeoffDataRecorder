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
    process::exit, 
    time::{
        SystemTime, 
        UNIX_EPOCH
    }
};

use rust_i18n::t;
use yansi::Paint;

use crate::{
    TODR_VERSION, 
    common::input::{
        get_string_from_input, 
        get_u8_from_input, 
        get_u64_from_input
    },
    database::{
        IdentityInfoT, 
        SingleTakeoffDataT, 
        TakeoffInfoT, 
        write::{
            create_fap_file, 
            write_fap_file
        }
    }, 
    yaml::serialization::data_to_yaml
};

pub fn record_to_database() {
    println!(" {}",t!("database.record.start").white());

    let identity_info = IdentityInfoT {
        name: get_string_from_input("database.record.input_name"),
        age:  get_u8_from_input("database.record.input_age"),
        long: get_u8_from_input("database.record.input_long"),
    };

    
    let raw_happ = get_u64_from_input("database.record.input_happiness");
    let happ = std::cmp::min(raw_happ, 100);

    if raw_happ > 100 {
        eprintln!(" {}", t!("database.record.happiness_over_limit"));
    }

    let takeoff_info = TakeoffInfoT {
        duration: get_u64_from_input("database.record.input_duration"),
        happiness: happ,
    };

    // Get UNIX time
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);

    // Building YAML struct
    let single_takeoff_data = SingleTakeoffDataT {
        version: TODR_VERSION.to_string(),
        time: time,
        identity: identity_info,
        takeoff: takeoff_info,
    };

    // Build file name and path and create .fap file
    let home = home::home_dir().unwrap_or_else(|| {
        eprintln!(" {}", t!("database.record.error_read_home"));
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });
    let dir = home.join("TakeoffDataRecorder");
    
    let filename = format!("{}-{}.fap", single_takeoff_data.identity.name, time);

    let data = match data_to_yaml(&single_takeoff_data) {
        Ok(data) => data,
        Err(error) => {
            eprintln!(" {}", error);
            return;
        }
    };

    println!("");

    let file_path = match create_fap_file(&dir, &filename) {
        Ok(file_path) => file_path,
        Err(error) => {
            eprintln!(" {}", error);
            exit(1);
        }
    };

    if let Err(error) = write_fap_file(&file_path, &data) {
        eprintln!(" {}", error);
        if let Err(error) = fs::remove_file(&file_path) {
                eprintln!(" {}", error);
                exit(1);
            }
        record_to_database();
    };
}
