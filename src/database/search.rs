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

use rust_i18n::t;

pub fn search_database_by_time(pairs: &[(i64, String, String)], time_str: &str) -> Vec<(i64, String, String)> {
    let mut results = Vec::new();

    for item in pairs {
        if item.2.contains(time_str) {
            results.push(item.clone());
        }
    }

    results
}

pub fn search_database_by_name(pairs: &[(i64, String, String)], name: &str) -> Vec<(i64, String, String)> {
    let mut results = Vec::new();

    for item in pairs {
        if item.1.contains(name) {
            results.push(item.clone());
        }
    }

    results
}

pub fn output_search_results(results: &[(i64, String, String)]) {
    if results.is_empty() {
        println!("{}", t!("database.search.no_found_result"));
        return;
    }

    let mut max_left = t!("database.collect.table_name").len();
    let mut max_right = t!("database.collect.table_time").len();

    for (_, left, right) in results {
        max_left = max_left.max(left.len());
        max_right = max_right.max(right.len());
    }

    println!(
        "{:<width_left$} | {:<width_right$}", 
        t!("database.collect.table_name"), t!("database.collect.table_time"), 
        width_left = max_left - 2, 
        width_right = max_right
    );
    println!(
        "{:-<width_left$}-+-{:-<width_right$}", 
        "", "", 
        width_left = max_left, 
        width_right = max_right
    );

    for (_, left, right) in results {
        println!(
            "{:<width_left$} | {:<width_right$}", 
            left, right,
            width_left = max_left, 
            width_right = max_right
        );
    }

    println!("");
}
