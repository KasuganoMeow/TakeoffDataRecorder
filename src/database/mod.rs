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

pub mod read;
pub mod write;

mod collect;
mod record;
mod search;

use rust_i18n::t;
use serde::{
    Deserialize, 
    Serialize
};
use yansi::Paint;

use crate::{console::{flush, readline}, database::{
    collect::{
        get_database, 
        output_database
    }, 
    record::record_to_database,
    search::{
        output_search_results, 
        search_by_name, 
        search_by_time
    }}};

#[derive(Deserialize, Serialize)]
pub struct SingleTakeoffDataT {
    pub version: String,
    pub time: u64,
    pub identity: IdentityInfoT,
    pub takeoff: TakeoffInfoT,
}

#[derive(Deserialize, Serialize)]
pub struct IdentityInfoT {
    pub name: String,
    pub age: u8,
    pub long: u8,
}

#[derive(Deserialize, Serialize)]
pub struct TakeoffInfoT {
    pub duration: u64,
    pub happiness: u64,
}

#[allow(dead_code)]
pub struct StatisticsT {
    pub file_count: u64,
    pub takeoff_stats: TakeoffStatsT,
    pub identityinfo_stats: IdentityInfoStatsT,
}

pub struct TakeoffStatsT {
    pub ave_duration: u64,
    pub ave_happiness: u64,
}

#[allow(dead_code)]
pub struct IdentityInfoStatsT {
    pub long_diff: u16,
}

pub fn record_data() {
    record_to_database();
}

pub fn list_database() {
    let (pairs, max_left, max_right) = get_database();
    output_database(&pairs, max_left, max_right);
}

pub fn search_database() {
    print!(" {} ", t!("database.what_search_type"));
    flush();
    match readline().as_str().trim() {
        "t" | "time" => {
            print!(" {}: ", t!("database.input_search_word"));
            flush();
            let time = readline().trim().to_string();
            println!("");
            let (pairs, _, _) = get_database();
            let results = search_by_time(&pairs, &time);
            output_search_results(&results);
        },
        "n" | "name" => {
            print!(" {}: ", t!("database.input_search_word"));
            flush();
            let name = readline().trim().to_string();
            println!("");
            let (pairs, _, _) = get_database();
            let results = search_by_name(&pairs, &name);
            output_search_results(&results);
        },
        "" => {
            println!(" {}", t!("basic.no_oper").bold().red());
            return;
        },
        _ => {
            println!(" {}", t!("basic.err_opt").bold().red());
            return;
        },
    }
}