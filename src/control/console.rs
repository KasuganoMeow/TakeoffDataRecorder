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

use crate::control::{input, output};
use crate::database;
use crate::services;

pub fn console_loop() {
    loop {
        print!(
            "{} {} {} ", 
            "ToDR".bold().white().blue(), 
            "0.1.0".bold().white().blue(),
            "/>".bold().green()
        );
        output::flush();

        let command = input::readline();

        if let Some(split_command) = command.as_str().trim().split_whitespace().next() {
            match split_command {
                "help" => exit(0),
                "record" => services::record::record_service_console(),
                "list" => services::collect::collect_service(),
                "search" => database::search_database(),
                "stats" => services::statistics::statistics_service_console(),
                "exit" => exit(0),
                _ => continue,
            }
        } else {
            continue;
        }
    }
}
