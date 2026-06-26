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

use std::sync::{
    LazyLock, 
    Mutex
};
use sys_locale::get_locale;

mod arguments;
mod common;
mod console;
mod data;
mod yaml;

rust_i18n::i18n!("locales", fallback = "en");

static TODR_VERSION: LazyLock<String> = LazyLock::new(|| {
    "0.1.0".to_string()
});

static IS_DEBUG: LazyLock<Mutex<bool>> = LazyLock::new(|| Mutex::new(false));

fn main() {
    // Get locale to initialization rust-i18n
    if let Some(locale) = get_locale() {
        rust_i18n::set_locale(&locale);
    } else {
        println!("Unable to read system language, fallback to en-US (en_US)");
    }
    
    arguments::processing::argument_processing();
}
