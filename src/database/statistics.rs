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

use crate::control;
use crate::database::{IdentityInfoStatsT, StatisticsT, TakeoffStatsT};
use crate::fap::deserialization;
use crate::utils::{directory, data};

pub fn get_statistics() -> Result<(String, StatisticsT), Box<dyn std::error::Error>> {
    print!("{}: ", t!("database.statistics.who"));
    control::output::flush();
    let name = control::input::readline().trim().to_string();

    let dir = directory::get_database_directory();

    let mut sum_duration: u64 = 0;
    let mut sum_happiness: u64 = 0;
    let mut long_max: u8 = 0;
    let mut long_min: u8 = 255;

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

    let range = fs::read_dir(dir)?;
    for entry in range {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(filename) = path.file_name().and_then(|s| s.to_str()) {                
                if filename.starts_with(&format!("{}-", name)) && (filename.ends_with(".fap")) {
                    let content = data::read_data_file(&path)?;
                    let data = deserialization::fap_to_data(&content);
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
        return Err(t!("database.statistics.error_user_file").into());
    }
    statistics.takeoff_stats.ave_duration = sum_duration / statistics.file_count;
    statistics.takeoff_stats.ave_happiness = sum_happiness / statistics.file_count;
    statistics.identityinfo_stats.long_diff = (long_max - long_min) as u16;

    
    Ok((name, statistics))
}
