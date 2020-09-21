#![allow(dead_code, unused_variables, unused_imports)]

#[cfg(feature = "client")]
pub mod client;
#[cfg(feature = "example")]
pub mod example;
pub mod models;
pub mod openapi_serialization;
pub mod security;
#[cfg(feature = "server")]
pub mod server;

pub use models::*;

pub const VERSION: &str = "{{info.version}}";
