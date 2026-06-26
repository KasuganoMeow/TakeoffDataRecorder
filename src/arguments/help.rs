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

use rust_i18n::t;
use yansi::Paint;

pub fn help() -> ! {
    println!(
        "{}: {} [{}]\n", 
        t!("basic.usage").bold().white(),
        "todr",
        t!("basic.option")
    );
    println!(
        "{}{}\n{}: {}\n",
        "record:  ".bold().blue(),
        t!("help_info.record"),
        t!("basic.usage"),
        "todr record".green()
    );
    println!(
        "{}{}\n{}: {}\n",
        "version: ".bold().blue(),
        t!("help_info.version"),
        t!("basic.usage"),
        "todr version".green()
    );
    exit(0);
}
