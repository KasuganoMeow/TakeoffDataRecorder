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

use rust_i18n::t;
use yansi::Paint;

use crate::{
    common::error::put_error, 
    database::{
        IdentityInfoStatsT, 
        StatisticsT, 
        TakeoffStatsT, 
        read::read_fap_file
    }, 
    yaml::deserialization::yaml_to_data
};

pub fn stats(name: String) -> ! {
    let home = home::home_dir().unwrap_or_else(|| {
        eprintln!("{}", t!("stats.err_home"));
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });
    let dir = home.join("TakeoffDataRecorder");

    let mut sum_duration: u64 = 0;
    let mut sum_happiness: u64 = 0;
    let mut long_max: u8 = 0;
    let mut long_min: u8 = 100;

    let identityinfo_stats = IdentityInfoStatsT {
        long_diff: 0,
    };

    let takeoff_stats = TakeoffStatsT {
        ave_duration: 0,
        ave_happiness: 0,
    };

    let mut statistics = StatisticsT {
        file_count: 0,
        takeoff_stats: takeoff_stats,
        identityinfo_stats: identityinfo_stats,
    };

    let range = match fs::read_dir(dir) {
        Ok(range) => range,
        Err(error) => {
            put_error(error.to_string());
            exit(1);
        }
    };
    for entry in range {
        let entry = match entry {
            Ok(entry) => entry,
            Err(error) => {
                put_error(error.to_string());
                exit(1);
            }
        };
        let path = entry.path();
        
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {                
                if filename.starts_with(&format!("{}-", name)) && (filename.ends_with(".fap")) {
                    let content = read_fap_file(&path);
                    let data = match yaml_to_data(&content) {
                        Ok(data) => data,
                        Err(error) => {
                            put_error(error.to_string());
                            stats(name);
                        }
                    };
                    if long_max < data.identity.long {
                        long_max = data.identity.long;
                    }
                    if long_min > data.identity.long {
                        long_min = data.identity.long
                    }
                    sum_duration += data.takeoff.duration;
                    sum_happiness += data.takeoff.happiness;
                    statistics.file_count += 1;
                }
            }
        }
    }
    if statistics.file_count == 0 {
        println!("\n{}:\n{}", t!("stats.err_user").bold().white(), name.bold().white());
        exit(1);
    }
    statistics.takeoff_stats.ave_duration = sum_duration / statistics.file_count;
    statistics.takeoff_stats.ave_happiness = sum_happiness / statistics.file_count;
    statistics.identityinfo_stats.long_diff = (long_max - long_min) as u16;

    println!("\n{}: {}", t!("stats.sum_file"), statistics.file_count);
    println!("{}{} {}s", name, t!("stats.ave_dura"), statistics.takeoff_stats.ave_duration);
    println!("{}{} {}%", name, t!("stats.ave_happ"), statistics.takeoff_stats.ave_happiness);
    println!("{}{} {}cm", name, t!("stats.long_dif"), statistics.identityinfo_stats.long_diff);
    exit(0);
}

pub fn help_stats() {
    println!(
        "{}{}\n{}: {}",
        "record:  ".bold().blue(),
        t!("help_info.record"),
        t!("basic.usage"),
        "todr stats".green()
    );
}
