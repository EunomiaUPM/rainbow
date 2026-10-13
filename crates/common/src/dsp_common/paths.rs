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

//! Where DSP 2025-1 is mounted on every agent; the DSP 10.2, 8.2 and 6.2 paths sit below.

/// Base of the current DSP version, as `.well-known/dspace-version` announces it.
pub const DSP_CURRENT: &str = "/dsp/current";
/// Transfer process endpoints (DSP 10.2).
pub const TRANSFERS: &str = "/dsp/current/transfers";
/// Contract negotiation endpoints (DSP 8.2).
pub const NEGOTIATIONS: &str = "/dsp/current/negotiations";
/// Catalog endpoints (DSP 6.2).
pub const CATALOG: &str = "/dsp/current/catalog";
