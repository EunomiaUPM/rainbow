/*
 * Copyright (C) 2026 - Universidad Politécnica de Madrid - UPM
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
//! Subscription commands: how an update treats the secret.

use common::secret::Secret;
use events::entities::commands::UpdateSubscriptionDto;

fn update_with(secret: &str) -> UpdateSubscriptionDto {
    UpdateSubscriptionDto {
        secret: Some(Secret::new(secret.to_string())),
        ..Default::default()
    }
}

/// An update that echoes the `*****` mask keeps the current secret.
#[test]
fn echoed_mask_keeps_the_current_secret() {
    assert!(update_with("*****").take_new_secret().is_none());
}

/// An update with a real value replaces the secret.
#[test]
fn new_value_replaces_the_secret() {
    let secret = update_with("n3w").take_new_secret().unwrap();
    assert_eq!(secret.expose(), "n3w");
}
