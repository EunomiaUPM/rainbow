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

use super::repos::{AuthInteractionRepoTrait, AuthRequestRepoTrait, AuthVerificationRepoTrait, MinionsRepoTrait};

pub trait RepoFactoryTrait: Send + Sync + Clone + 'static {
    type RequestRepo: AuthRequestRepoTrait + Send + Sync + Clone + 'static;
    type InteractionRepo: AuthInteractionRepoTrait + Send + Sync + Clone + 'static;
    type VerificationRepo: AuthVerificationRepoTrait + Send + Sync + Clone + 'static;
    type MinionsRepo: MinionsRepoTrait + Send + Sync + Clone + 'static;

    fn request(&self) -> &Self::RequestRepo;
    fn interaction(&self) -> &Self::InteractionRepo;
    fn verification(&self) -> &Self::VerificationRepo;
    fn minions(&self) -> &Self::MinionsRepo;
}
