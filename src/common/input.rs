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

use std::io::{
    self, 
    Write
};

use rust_i18n::t;
use yansi::Paint;

use crate::common::error::put_error;

pub fn get_string_from_input(translate: &'static str) -> String {
    let string = loop {
        let mut buffer = String::new();
        print!(" {}: ", t!(translate).white());
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
    string
}

pub fn get_u8_from_input(translate: &'static str) -> u8 {
    let u8 = loop {
        let mut buffer = String::new();
        print!(" {}: ", t!(translate).white());
        if let Err(e) = io::stdout().flush() {
            put_error(e.to_string());
            continue;
        }

        if let Err(e) = io::stdin().read_line(&mut buffer) {
            put_error(e.to_string());
            continue;
        }

        match buffer.trim().parse::<u8>() {
            Ok(num) => {
                break num;
            }
            Err(e) => {
                put_error(e.to_string());
                continue;
            }
        }
    };
    u8
}

pub fn get_u64_from_input(translate: &'static str) -> u64 {
    let u64 = loop {
        let mut buffer = String::new();
        print!(" {}: ", t!(translate).white());
        if let Err(e) = io::stdout().flush() {
            put_error(e.to_string());
            continue;
        }

        if let Err(e) = io::stdin().read_line(&mut buffer) {
            put_error(e.to_string());
            continue;
        }

        match buffer.trim().parse::<u64>() {
            Ok(num) => {
                break num;
            }
            Err(e) => {
                put_error(e.to_string());
                continue;
            }
        }
    };
    u64
}
