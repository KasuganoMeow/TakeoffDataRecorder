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

use rust_i18n::t;

use crate::control::{input, output};
use crate::database::{IdentityInfoT, SingleTakeoffDataT, TakeoffInfoT};
use crate::utils::time;
use crate::fap::serialization;

pub fn record_database() -> Result<(String, String), Box<dyn std::error::Error>> {
    println!("{}",t!("database.record.record_start"));
    let identity_info = IdentityInfoT {
        name: {
            print!("{}: ", t!("database.record.input_name"));
            output::flush();
            let input = input::readline().trim().to_string();
            if input.is_empty() {
                "ANONYMOUS".to_string()
            } else {
                input
            }
        },
        age: {
            print!("{}: ", t!("database.record.input_age"));
            output::flush();
            match input::readline().trim().parse() {
                Ok(age) => age,
                Err(error) => {
                    eprintln!("{}", error);
                    0
                }
            }
        },
        long: {
            print!("{}: ", t!("database.record.input_long"));
            output::flush();
            match input::readline().trim().parse() {
                Ok(long) => long,
                Err(error) => {
                    eprintln!("{}", error);
                    0
                }
            }
        }
    };

    let takeoff_info = TakeoffInfoT {
        duration: {
            print!("{}: ", t!("database.record.input_duration"));
            output::flush();
            match input::readline().trim().parse() {
                Ok(duration) => duration,
                Err(error) => {
                    eprintln!("{}", error);
                    0
                }
            }
        },
        happiness: {
            print!("{}: ", t!("database.record.input_happiness"));
            output::flush();
            match input::readline().trim().parse() {
                Ok(happiness) => {
                    if happiness > 100 {
                        eprintln!("{}", t!("database.record.happiness_over_limit"));
                        100
                    } else {
                        happiness
                    }
                }
                Err(error) => {
                    eprintln!("{}", error);
                    0
                }
            }
        }
    };

    let single_takeoff_data = SingleTakeoffDataT {
        version: "0.1.0".to_string(),
        timestamp: time::get_unix_timestamp(),
        identity: identity_info,
        takeoff: takeoff_info,
    };

    let content = serialization::data_to_fap(&single_takeoff_data);

    Ok((single_takeoff_data.identity.name, content))
}
