//! Common modules shared by both client and server.

#![deny(warnings)]

extern crate "rustc-serialize" as rustc_serialize;

/// Codename of the project.
pub const PROJECT_NAME: &'static str = "Project Fate";

#[test]
fn it_works() {
}

pub mod message;
