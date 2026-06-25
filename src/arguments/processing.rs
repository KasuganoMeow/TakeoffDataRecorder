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

use rust_i18n::t;

use crate::{
    IS_DEBUG, 
    arguments::{
        help::help, 
        record::record, 
        version::version
    }, 
    common::error
};

pub fn argument_processing() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error::put_error(t!("basic.no_oper").to_string());
        return;
    }

    for arg in &args {
        if arg == "debug" {
            *IS_DEBUG.lock().unwrap() = true;
        }
    }

    match args[1].as_str() {
        "help"    => help(),
        "record"  => record(),
        "version" => version(),
        _         => {
            error::put_error(t!("basic.err_opt").to_string());
            return;
        },
    }
}
