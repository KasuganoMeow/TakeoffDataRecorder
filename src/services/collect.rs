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

use std::env;

use crate::control::output;
use crate::database::collect;

pub fn collect_service() {
    let (pairs, max_left, max_right) = match collect::get_database() {
        Ok((pairs, max_left, max_right)) => (pairs, max_left, max_right),
        Err(error) => {
            eprintln!("{}\n", error);
            return;
        }
    };
    let args: Vec<String> = env::args().collect();
    if args[1].as_str() == "console" {
        match output::output_database_console(&pairs, max_left, max_right) {
            Ok(()) => return,
            Err(error) => {
                eprintln!("{}\n", error);
                return;
            }
        };
    } else if args[1].as_str() == "direct" {
        match output::output_database_direct(&pairs, max_left, max_right) {
            Ok(()) => return,
            Err(error) => {
                eprintln!("{}\n", error);
                return;
            }
        };
    }
}
