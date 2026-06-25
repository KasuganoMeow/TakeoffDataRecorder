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
    fs,
    path::{
        Path, 
        PathBuf
    }
};

use rust_i18n::t;
use yansi::Paint;

pub fn create_fap_file(dir: PathBuf, filename: String) -> PathBuf {
    let file_path = if dir.exists() || std::fs::create_dir_all(&dir).is_ok() {
        Path::new(&dir).join(&filename)
    } else {
        eprintln!("{}", t!("data.err_mdir"));
        Path::new(".").join(&filename)
    };
    
    fs::write(&file_path, "")
        .expect(&format!("{}", t!("data.err_crea").bold().red()));
    println!("{}: {}", t!("data.create").bold().white(), file_path.display().green());

    return file_path;
}

pub fn write_fap_file(file_path: PathBuf, data: String) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&file_path, data)?;
    println!("{}: {}", t!("data.write").bold().white(), file_path.display().green());

    Ok(())
}
