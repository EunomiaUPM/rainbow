mod config;
mod db;
mod entities;
mod errors;
mod grpc;
mod http;
mod protocols;
mod services;
pub mod setup;
mod tests;

pub use db::get_transfer_agent_migrations;
pub use setup::create_root_http_router as create_transfer_agente_router;

pub trait TransferDummyTrait {}
