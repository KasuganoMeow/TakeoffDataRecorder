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
    fs, io::{
        self, 
        Write
    }, path::PathBuf, process::exit, time::{
        SystemTime, 
        UNIX_EPOCH
    }
};

use rust_i18n::t;
use yansi::Paint;

use crate::{
    TODR_VERSION, 
    common::error::put_error,
    data::{
        IdentityInfoT, 
        SingleTakeoffDataT, 
        TakeoffInfoT, write::{create_fap_file, write_fap_file}
    }, 
    yaml::serialization::data_to_yaml
};

pub fn record() -> ! {
    println!("{}",t!("record.to_star").bold().white());

    let mut identity_info = IdentityInfoT {
        name: "default".to_string(),
        age:  0,
        long: 0,
    };

    let mut takeoff_info = TakeoffInfoT {
        duration: 0,
        happiness: 0,
    };

    let mut buffer = String::new();
    
    // Name
    'name: loop {
        print!("{}: ", t!("record.to_name").blue());
        if let Err(e) = io::stdout().flush() {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'name;
        }

        if let Err(e) = io::stdin().read_line(&mut buffer) {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'name;
        }

        if buffer == "\n" {
            identity_info.name = "ANONYMOUS".to_string();
            String::clear(&mut buffer);
            break 'name;
        }

        identity_info.name = buffer.clone().trim().to_string();

        String::clear(&mut buffer);

        break 'name;
    }

    // Age
    'age: loop {
        print!("{}: ", t!("record.to_age").blue());
        if let Err(e) = io::stdout().flush() {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'age;
        }

        if let Err(e) = io::stdin().read_line(&mut buffer) {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'age;
        }

        match buffer.trim().parse::<u8>() {
            Ok(age) => {
                identity_info.age = age;
                String::clear(&mut buffer);
                break 'age;
            }
            Err(e) => {
                put_error(e.to_string());
                String::clear(&mut buffer);
                continue 'age;
            }
        }
    }

    // Long
    'long: loop {
        print!("{}: ", t!("record.to_long").blue());
        if let Err(e) = io::stdout().flush() {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'long;
        }

        if let Err(e) = io::stdin().read_line(&mut buffer) {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'long;
        }

        match buffer.trim().parse::<u8>() {
            Ok(long) => {
                identity_info.long = long;
                String::clear(&mut buffer);
                break 'long;
            }
            Err(e) => {
                put_error(e.to_string());
                String::clear(&mut buffer);
                continue 'long;
            }
        }
    }

    // Duration
    'duration: loop {
        print!("{}: ", t!("record.to_dura").blue());
        if let Err(e) = io::stdout().flush() {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'duration;
        }

        if let Err(e) = io::stdin().read_line(&mut buffer) {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'duration;
        }

        match buffer.trim().parse::<u64>() {
            Ok(duration) => {
                takeoff_info.duration = duration;
                String::clear(&mut buffer);
                break 'duration;
            }
            Err(e) => {
                put_error(e.to_string());
                String::clear(&mut buffer);
                continue 'duration;
            }
        }
    }

    // Happiness
    'happiness: loop {
        print!("{}: ", t!("record.to_happ").blue());
        if let Err(e) = io::stdout().flush() {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'happiness;
        }

        if let Err(e) = io::stdin().read_line(&mut buffer) {
            put_error(e.to_string());
            String::clear(&mut buffer);
            continue 'happiness;
        }

        match buffer.trim().parse::<u64>() {
            Ok(happiness) => {
                takeoff_info.happiness = happiness;
                String::clear(&mut buffer);
                break 'happiness;
            }
            Err(e) => {
                put_error(e.to_string());
                String::clear(&mut buffer);
                continue 'happiness;
            }
        }
    }

    String::clear(&mut buffer);

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
        eprintln!("{}", t!("record.err_home"));
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });
    let dir = home.join("TakeoffDataRecorder");
    
    let filename = format!("{}-{}.fap", single_takeoff_data.identity.name, time);

    let data = match data_to_yaml(&single_takeoff_data) {
        Ok(d) => d,
        Err(e) => {
            put_error(e.to_string());
            record();
        }
    };

    let file_path = match create_fap_file(&dir, &filename) {
        Ok(fp) => fp,
        Err(e) => {
            put_error(e.to_string());
            exit(1);
        }
    };

    if let Err(e) = write_fap_file(&file_path, &data) {
        put_error(e.to_string());
        if let Err(e) = fs::remove_file(&file_path) {
                put_error(e.to_string());
                exit(1);
            }
        record();
    };

    exit(0);
}
