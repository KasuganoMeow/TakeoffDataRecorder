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

use std::{io::Error, process::exit};

use rust_i18n::t;
use yansi::Paint;

pub fn put_error(error: String) {
    eprintln!(
        "{}: {}: {}",
        "todr".bold().white(),
        t!("basic.error").bold().white(),
        error.bold().red()
    );
}

pub fn io_error(error: Error) {
    eprintln!(
        "[ToDR]: {}: {}", 
        t!("basic.err_fatl"),
        error
    );
    exit(74);
}
