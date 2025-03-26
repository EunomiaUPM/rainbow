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

use anyhow::bail;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use log::error;
use rainbow_common::config::config::get_provider_portal_url;
use rand::{distributions::Alphanumeric, Rng};
use tracing::info;
use urlencoding::{decode, encode};

pub struct Manager {}
impl Manager {
    pub fn generate_exchange_uri() -> String {
        generate_openid4vp_uri()
    }
}

fn generate_openid4vp_uri() -> String {
    let base_url = "openid4vp://authorize";
    let provider_url = get_provider_portal_url().unwrap();

    // Cliente (verificador)
    let client_id = format!("{}/verify", &provider_url);
    let encoded_client_id = encode(&client_id);

    // Generamos valores aleatorios
    let state: String =
        rand::thread_rng().sample_iter(&Alphanumeric).take(12).map(char::from).collect();

    let nonce: String =
        rand::thread_rng().sample_iter(&Alphanumeric).take(36).map(char::from).collect();

    // COMPLETAR
    let presentation_definition_uri = format!("{}/pd/{}", &provider_url, state);
    let encoded_presentation_definition_uri = encode(&presentation_definition_uri);

    // COMPLETAR
    // let presentation_definition_uri = format!("https://rainbow/openid4vc/pd/{}", state);

    // COMPLETAR
    let response_uri = format!("{}/verify/{}", &provider_url, state);
    let encoded_response_uri = encode(&response_uri);

    let response_type = "vptoken";
    let response_mode = "direct_post";
    let clientid_scheme = "redirect_uri";

    let client_metadata = r#"{"authorization_encrypted_response_alg":"ECDH-ES","authorization_encrypted_response_enc":"A256GCM"}"#;

    let uri = format!("{}?response_type={}&client_id={}&response_mode={}&presentation_definition_uri={}&client_id_scheme={}&client_metadata={}&nonce={}&response_uri={}", base_url, response_type, encoded_client_id, response_mode, encoded_presentation_definition_uri, clientid_scheme, client_metadata, nonce, encoded_response_uri);

    uri
}
