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

pub fn create_fap_file(dir: &PathBuf, filename: &String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = if dir.exists() || std::fs::create_dir_all(&dir).is_ok() {
        Path::new(&dir).join(&filename)
    } else {
        eprintln!(" {}", t!("database.write.error_mkdir"));
        Path::new(".").join(&filename)
    };
    
    fs::write(&path, "")
        .expect(&format!(" {}", t!("database.write.error_create_file").bold().red()));
    println!(" {}: {}", t!("database.write.create_file").white(), path.display().green());

    Ok(path)
}

pub fn write_fap_file(path: &PathBuf, yaml: &String) -> Result<(), Box<dyn std::error::Error>> {
    fs::write(&path, yaml)?;
    println!(" {}: {}", t!("database.write.write_file").white(), path.display().green());

    Ok(())
}
