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

use crate::ssi_auth::consumer::SSI_AUTH_HTTP_CLIENT;
use anyhow::bail;
use axum::http::StatusCode;
use rainbow_common::config::config::{get_consumer_ssi_holder, GLOBAL_CONFIG};
use tracing::debug;

pub type ConsumerSSIVCRequest = serde_json::Value;

pub async fn consumer_vc_request(input: ConsumerSSIVCRequest) -> anyhow::Result<()> {
    let ssi_consumer_url = get_consumer_ssi_holder()?;
    let res = SSI_AUTH_HTTP_CLIENT
        .post(ssi_consumer_url)
        .json(&input)
        .send()
        .await
        .map_err(|_| bail!("SSI holder not available"))?;

    match res.status() {
        StatusCode::CREATED => {
            let vc = res.text().await?;
            debug!(vc);
            Ok(())
        }
        _ => bail!("Credential not issued"),
    }
}

pub async fn consumer_wf_exchange_from_consumer() -> anyhow::Result<()> {
    Ok(())
}
