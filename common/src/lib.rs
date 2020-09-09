//! Common modules shared by both client and server.

#![deny(warnings)]

extern crate log;
extern crate rustc_serialize;

/// Codename of the project.
pub const PROJECT_NAME: &str = "Project Fate";

pub mod manager;
pub mod message;
pub mod simple_logger;
