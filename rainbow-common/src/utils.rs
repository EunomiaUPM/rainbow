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

use uuid::Uuid;
use urn::Urn;


static UUID_PREFIX: &str ="urn:uuid:";

pub fn convert_uuid_to_uri(uuid: &Uuid) -> anyhow::Result<String> {
    Ok(format!("urn:uuid:{}", uuid.to_string()))
}

pub fn convert_uri_to_uuid(string: &String) -> anyhow::Result<Uuid> {
    let string = string.replace("urn:uuid:", "");
    let uuid = Uuid::parse_str(&string)?;
    Ok(uuid)
}

pub fn get_urn(optional_urn: Option<Urn>) -> Urn {
    let uuid = Uuid::new_v4();
    let id_string =  UUID_PREFIX.to_string() + &uuid.to_string();
    let mut urn_res = id_string.parse::<Urn>().unwrap();
    if !optional_urn.is_none() {
        urn_res = optional_urn.unwrap();
    }
    return  urn_res;
}