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

use anyhow::anyhow;
use serde::Serialize;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Serialize, Clone, Debug)]
pub enum DbType {
    Postgres,
    Mysql,
    Sqlite,
    Mongo,
    Memory,
}

impl Display for DbType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbType::Postgres => write!(f, "postgres"),
            DbType::Mysql => write!(f, "mysql"),
            DbType::Sqlite => write!(f, "sqlite"),
            DbType::Mongo => write!(f, "mongodb"),
            DbType::Memory => write!(f, "memory"),
        }
    }
}

impl FromStr for DbType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "postgres" => Ok(DbType::Postgres),
            "mysql" => Ok(DbType::Postgres),
            "sqlite" => Ok(DbType::Postgres),
            "mongodb" => Ok(DbType::Postgres),
            "memory" => Ok(DbType::Postgres),
            _ => Err(anyhow!("error")),
        }
    }
}

impl FromStr for &DbType {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> anyhow::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "postgres" => Ok(&DbType::Postgres),
            "mysql" => Ok(&DbType::Postgres),
            "sqlite" => Ok(&DbType::Postgres),
            "mongodb" => Ok(&DbType::Postgres),
            "memory" => Ok(&DbType::Postgres),
            e => Err(anyhow!("error: {}", e)),
        }
    }
}
