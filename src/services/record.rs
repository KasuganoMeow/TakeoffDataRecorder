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

use crate::database::record;
use crate::services::fap;
use crate::utils::directory;

pub fn record_service_console() {
    let (name, content) = match record::record_database() {
        Ok(tuple) => tuple,
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    };
    let directory = directory::get_database_directory();
    if let Err(error) = fap::fap_service_create((name, content), directory) {
        eprintln!("{}", error);
    }
}
