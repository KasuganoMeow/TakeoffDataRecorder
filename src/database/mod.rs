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

pub mod collect;
pub mod record;
mod search;
pub mod statistics;

use rust_i18n::t;
use serde::{Deserialize, Serialize};
use yansi::Paint;

use crate::control::{input, output};

#[derive(Default, Deserialize, Serialize)]
pub struct SingleTakeoffDataT {
    pub version: String,
    pub timestamp: i64,
    pub identity: IdentityInfoT,
    pub takeoff: TakeoffInfoT,
}

#[derive(Default, Deserialize, Serialize)]
pub struct IdentityInfoT {
    pub name: String,
    pub age: u8,
    pub long: u8,
}

#[derive(Default, Deserialize, Serialize)]
pub struct TakeoffInfoT {
    pub duration: u64,
    pub happiness: u64,
}

#[derive(Default, Deserialize, Serialize)]
pub struct StatisticsT {
    pub file_count: u64,
    pub takeoff_stats: TakeoffStatsT,
    pub identityinfo_stats: IdentityInfoStatsT,
}

#[derive(Default, Deserialize, Serialize)]
pub struct TakeoffStatsT {
    pub ave_duration: u64,
    pub ave_happiness: u64,
}

#[derive(Default, Deserialize, Serialize)]
pub struct IdentityInfoStatsT {
    pub long_diff: u16,
}

pub fn search_database() {
    print!("{}: ", t!("database.what_search_type"));
    output::flush();
    match input::readline().as_str().trim() {
        "t" | "time" => {
            print!("{}: ", t!("database.input_search_word"));
            output::flush();
            let time = input::readline().trim().to_string();
            println!("");
            let (pairs, _, _) = match collect::get_database() {
                Ok((pairs, _1, _2)) => (pairs, _1, _2),
                Err(error) => {
                    eprintln!("{}\n", error);
                    return;
                }
            };
            let results = search::search_database_by_time(&pairs, &time);
            search::output_search_results(&results);
        },
        "n" | "name" => {
            print!("{}: ", t!("database.input_search_word"));
            output::flush();
            let name = input::readline().trim().to_string();
            println!("");
            let (pairs, _, _) = match collect::get_database() {
                Ok((pairs, _1, _2)) => (pairs, _1, _2),
                Err(error) => {
                    eprintln!("{}\n", error);
                    return;
                }
            };
            let results = search::search_database_by_name(&pairs, &name);
            search::output_search_results(&results);
        },
        "" => {
            println!("{}", t!("basic.no_oper").bold().red());
            return;
        },
        _ => {
            println!("{}", t!("basic.err_opt").bold().red());
            return;
        },
    }
}
