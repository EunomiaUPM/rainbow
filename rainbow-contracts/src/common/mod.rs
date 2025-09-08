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

use std::fmt::{Display, Formatter};

pub mod schemas;
pub mod core;

pub enum CNControllerTypes {
    Process,
    Message,
    Offer,
    Agreement,
    Participant,
}
impl Display for CNControllerTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CNControllerTypes::Process => write!(f, "Contract Negotiation Process"),
            CNControllerTypes::Message => write!(f, "Contract Negotiation Message"),
            CNControllerTypes::Offer => write!(f, "Contract Negotiation Offer"),
            CNControllerTypes::Agreement => write!(f, "Contract Negotiation Agreement"),
            CNControllerTypes::Participant => write!(f, "Contract Negotiation Participant"),
        }
    }
}
