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

use serde::{
    Deserialize, 
    Serialize
};

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
    pub long_grow_rate: u16,
}
