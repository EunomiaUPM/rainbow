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
use crate::config::config::get_local_database_url;
use anyhow::anyhow;
use sea_orm::{Database, DatabaseConnection};
use serde::Serialize;
use std::fmt::Display;
use std::str::FromStr;
use tokio::sync::OnceCell;
use tracing::info;

pub static DB_CONNECTION: OnceCell<DatabaseConnection> = OnceCell::const_new();
pub async fn get_db_connection() -> &'static DatabaseConnection {
    DB_CONNECTION
        .get_or_init(|| async {
            let db_url = get_local_database_url().unwrap();
            let db = Database::connect(db_url).await;
            match db {
                Ok(db) => {
                    info!("Database connection successfully established");
                    db
                }
                Err(e) => {
                    panic!("Database connection error: {}", e.to_string())
                }
            }
        })
        .await
}

#[derive(Serialize, Clone)]
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
            _ => Err(anyhow!("error"))
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
            e => Err(anyhow!("error: {}", e))
        }
    }
}
