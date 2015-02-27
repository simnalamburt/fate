//! Common modules shared by both client and server.

#![deny(warnings)]

extern crate "rustc-serialize" as rustc_serialize;
extern crate log;

/// Codename of the project.
pub const PROJECT_NAME: &'static str = "Project Fate";

pub mod message;
pub mod simple_logger;
