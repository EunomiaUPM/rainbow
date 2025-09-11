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

use super::traits::{AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthVerificationRepoTrait, MinionsRepoTrait};
use std::sync::Arc;

pub trait AuthRepoFactoryTrait: Send + Sync + Clone + 'static {
    fn request(&self) -> Arc<dyn AuthRequestRepoTrait>;
    fn interaction(&self) -> Arc<dyn AuthInteractionRepoTrait>;
    fn verification(&self) -> Arc<dyn AuthVerificationRepoTrait>;
    fn minions(&self) -> Arc<dyn MinionsRepoTrait>;
}
