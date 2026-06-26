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

use std::{
    env, 
    process::exit
};

use rust_i18n::t;

use crate::{
    IS_DEBUG, 
    arguments::{
        help::help, 
        record::{
            help_record, 
            record
        }, 
        stats::{
            help_stats, 
            stats
        }, 
        version::{
            help_version, 
            version
        }
    }, 
    common::{
        error, 
        input::get_string_from_input
    }, console::todr_console
};

pub fn argument_processing() -> ! {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        error::put_error(t!("basic.no_oper").to_string());
        exit(1);
    }

    for arg in &args {
        if arg == "debug" {
            *IS_DEBUG.lock().unwrap() = true;
        }
    }

    if args.len() > 2 {
        match args[1].as_str() {
            "help"    => {
                match args[2].as_str() {
                    "record" => {
                        help_record();
                        exit(0);
                    },
                    "stats" => {
                        help_stats();
                        exit(0);
                    },
                    "version" => {
                        help_version();
                        exit(0);
                    },
                    _ => {
                        error::put_error(t!("basic.err_opt").to_string());
                        exit(1);
                    },
                }
            },
            "record" => record(),
            "stats" => {
                stats(args[2].trim().to_string());
            },
            "version" => version(),
            _ => {
                error::put_error(t!("basic.err_opt").to_string());
                exit(1);
            },
        }
    } else {
        match args[1].as_str() {
            "help" => help(),
            "record" => record(),
            "stats" => stats(
                get_string_from_input("common.rto_name"),
            ),
            "version" => version(),
            "console" => todr_console(),
            _ => {
                error::put_error(t!("basic.err_opt").to_string());
                exit(1);
            },
        }
    }
}
