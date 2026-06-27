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

use std::io;

use rust_i18n::t;

use crate::database::StatisticsT;
use crate::utils;

pub fn flush() {
    if let Err(error) = io::Write::flush(&mut io::stdout()) {
        eprintln!("{}", error);
    }
}

pub fn output_stats(name: String, statistics: StatisticsT) {
    println!("\n{}: {}", t!("control.output.statistics.sum_of_file"), statistics.file_count);
    println!("{}{} {}s", name, t!("control.output.statistics.average_of_duration"), statistics.takeoff_stats.ave_duration);
    println!("{}{} {}%", name, t!("control.output.statistics.average_of_happiness"), statistics.takeoff_stats.ave_happiness);
    println!("{}{} {}cm\n", name, t!("control.output.statistics.long_difference"), statistics.identityinfo_stats.long_diff);
}

pub fn output_database_console(pairs: &[(i64, String, String)], max_left: usize, max_right: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}\n{}:\n", t!("control.output.collect.found_file"), utils::directory::get_database_directory().to_string_lossy());
    println!(
        " {:<width_left$} | {:<width_right$}", 
        t!("control.output.collect.table_name"), t!("control.output.collect.table_time"), 
        width_left = max_left - 2, 
        width_right = max_right
    );
    println!(
        "-{:-<width_left$}-+-{:-<width_right$}-", 
        "", "", 
        width_left = max_left, 
        width_right = max_right
    );
    for (_, left, right) in pairs {
        println!(
            " {:<width_left$} | {:<width_right$}", 
            left, right,
            width_left = max_left, 
            width_right = max_right
        );
    }
    println!("");
    Ok(())
}

pub fn output_database_direct(pairs: &[(i64, String, String)], max_left: usize, max_right: usize) -> Result<(), Box<dyn std::error::Error>> {
    for (_, left, right) in pairs {
        println!(
            "{:<width_left$} {:<width_right$}", 
            left, right,
            width_left = max_left, 
            width_right = max_right
        );
    }
    println!("");
    Ok(())
}
