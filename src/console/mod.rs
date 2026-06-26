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


use crate::{
    console::conloop::console_loop
};

mod conloop;

pub fn todr_console() -> ! {
    console_loop();
}

pub fn flush() {
    if let Err(error) = std::io::Write::flush(&mut std::io::stdout()) {
        crate::common::error::io_error(error);
    }
}

pub fn readline() -> String{
    let mut buffer = String::new();
    if let Err(error) = std::io::stdin().read_line(&mut buffer) {
        crate::common::error::io_error(error);
    }
    buffer
}

