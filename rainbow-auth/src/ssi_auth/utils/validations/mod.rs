/*
 *
 *  * Copyright (C) 2024 - Universidad Politécnica de Madrid - UPM
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
use chrono::{DateTime, Utc};

pub fn compare_with_margin(iat: i64, issuance_date: &str, margin_seconds: i64) -> (bool, String) {
    let datetime = match DateTime::from_timestamp(iat, 0) {
        Some(dt) => dt,
        None => return (true, "Invalid iat field".to_string()),
    };

    let parsed_date = match DateTime::parse_from_rfc3339(issuance_date) {
        Ok(dt) => dt,
        Err(_) => {
            return (
                true,
                "IssuanceDate is not with the correct format".to_string(),
            )
        }
    };
    let parsed_date_utc = parsed_date.with_timezone(&Utc);

    if parsed_date_utc > Utc::now() {
        return (true, "Issuance date has not reached yet".to_string());
    }

    if (datetime - parsed_date_utc).num_seconds().abs() > margin_seconds {
        return (true, "IssuanceDate & iat field do not match".to_string());
    }

    (false, "Ignore this".to_string())
}
