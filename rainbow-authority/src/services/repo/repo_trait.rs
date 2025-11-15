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

use super::subtraits::{RequestRepoTrait, InteractionRepoTrait, VerificationRepoTrait, IssuingRepoTrait, MinionsRepoTrait};
use std::sync::Arc;

pub trait RepoTrait: Send + Sync + 'static {
    fn request(&self) -> Arc<dyn RequestRepoTrait>;
    fn interaction(&self) -> Arc<dyn InteractionRepoTrait>;
    fn verification(&self) -> Arc<dyn VerificationRepoTrait>;
    fn minions(&self) -> Arc<dyn MinionsRepoTrait>;
    fn issuing(&self) -> Arc<dyn IssuingRepoTrait>;
}
