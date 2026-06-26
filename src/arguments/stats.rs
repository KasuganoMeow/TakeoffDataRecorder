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
    fs, io::{self, Write}, path::PathBuf, process::exit
};

use rust_i18n::t;
use yansi::Paint;

use crate::{
    common::error::put_error, 
    data::{
        IdentityInfoStatsT, 
        StatisticsT, 
        TakeoffStatsT, 
        read::read_fap_file
    }, 
    yaml::deserialization::yaml_to_data
};

pub fn stats() -> ! {
    // Name
    let name = loop {
        let mut buffer = String::new();
        print!("{}: ", t!("stats.to_name").bold().white());
        if let Err(e) = io::stdout().flush() {
            put_error(e.to_string());
            continue;
        }

        if let Err(e) = io::stdin().read_line(&mut buffer) {
            put_error(e.to_string());
            continue;
        }

        if buffer.trim().is_empty() {
            break "ANONYMOUS".to_string();
        }

        break buffer.trim().to_string();
    };

    let home = home::home_dir().unwrap_or_else(|| {
        eprintln!("{}", t!("stats.err_home"));
        std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
    });
    let dir = home.join("TakeoffDataRecorder");

    let mut sum_duration: u64 = 0;
    let mut sum_happiness: u64 = 0;

    let identityinfo_stats = IdentityInfoStatsT {
        long_grow_rate: 0,
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
                    let content = match read_fap_file(&path) {
                        Ok(content) => content,
                        Err(error) => {
                            put_error(error.to_string());
                            exit(1);
                        }
                    };
                    let data = match yaml_to_data(&content) {
                        Ok(data) => data,
                        Err(error) => {
                            put_error(error.to_string());
                            stats()
                        }
                    };
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
    println!("\n总共读取了 {} 个文件", statistics.file_count);
    println!("{} 的平均 \"起飞\" 持续时间为 {} 秒", name, statistics.takeoff_stats.ave_duration);
    println!("{} 的平均 \"起飞\" 满意度为 {}%", name, statistics.takeoff_stats.ave_happiness);
    exit(0);
}
