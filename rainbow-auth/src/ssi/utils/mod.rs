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
use anyhow::bail;
use rainbow_common::errors::helpers::BadFormat;
use rainbow_common::errors::{CommonErrors, ErrorLog};
use std::fs;
use tracing::error;
use url::Url;

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

pub fn get_query_param(parsed_uri: &Url, param_name: &str) -> anyhow::Result<String> {
    if let Some(value) = parsed_uri.query_pairs().find(|(k, _)| k == param_name).map(|(_, v)| v.into_owned()) {
        Ok(value)
    } else {
        let error = CommonErrors::format_new(
            BadFormat::Received,
            &format!(
                "The expected '{}' field was missing in the oidc4vp uri",
                param_name
            ),
        );
        error!("{}", error.log());
        bail!(error);
    }
}
