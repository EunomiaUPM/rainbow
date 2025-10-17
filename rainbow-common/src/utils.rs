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

use crate::errors::helpers::BadFormat;
use crate::errors::{CommonErrors, ErrorLog};
use anyhow::bail;
use tracing::error;
use urn::Urn;
use uuid::Uuid;

static UUID_PREFIX: &str = "urn:uuid:";

pub fn get_urn(optional_urn: Option<Urn>) -> Urn {
    optional_urn.unwrap_or_else(|| {
        let uuid = Uuid::new_v4();
        let id_string = format!("{}{}", UUID_PREFIX, uuid);
        let urn = id_string.parse::<Urn>().unwrap();
        urn
    })
}

pub fn get_urn_from_string(string_in: &String) -> anyhow::Result<Urn> {
    let urn_res = string_in.parse::<Urn>();
    match urn_res {
        Ok(urn_res) => Ok(urn_res),
        Err(e) => {
            let e_ = CommonErrors::format_new(BadFormat::Received, e.to_string().into());
            error!("{}", e_.log());
            bail!(e_);
        }
    }
}
