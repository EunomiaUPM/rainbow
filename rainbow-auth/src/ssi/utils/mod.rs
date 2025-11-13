/*
 *
 *  * Copyright (C) 2025 - Universidad Politécnica de Madrid - UPM
 *  *
 *  * This program is free software: you can redistribute it and/or modify
 *  * it under the terms of the GNU General Public License as published by
 *  * the Free Software Foundation, either version 3 of the License, or
 *  * (at your option) any later version.
 *  *
 *  * This program is distributed in the hope that it will be useful,
 *  * but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  * GNU General Public License for more details.
 *  *
 *  * You should have received a copy of the GNU General Public License
 *  * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 *
 */
use std::fs;
use anyhow::bail;
use tracing::error;
use rainbow_common::errors::CommonErrors;

pub fn read(path: &str) -> anyhow::Result<String> {
    match fs::read_to_string(&path) {
        Ok(data) => Ok(data),
        Err(e) => {
            let error = CommonErrors::read_new(path, &e.to_string());
            error!("{}", error);
            bail!(error)
        }
    }
}