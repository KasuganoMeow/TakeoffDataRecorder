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

use std::process::exit;

use yansi::Paint;

use crate::{
    TODR_VERSION, 
    console::{
        flush, 
        readline
    }, 
    database::{
        list_database, 
        record_data, 
        search_database
    }, 
};

pub fn console_loop() -> ! {
    let version = TODR_VERSION.to_string();
    loop {
        print!(
            "{} {} {} ", 
            "ToDR".bold().white().blue(), 
            version.bold().white().blue(),
            "/>".bold().green()
        );
        flush();

        match readline().as_str().trim() {
            "help" => crate::arguments::help::help(),
            "record" => record_data(),
            "list" => list_database(),
            "search" => search_database(),
            "exit" => exit(0),
            _ => continue,
        }
    }
}
