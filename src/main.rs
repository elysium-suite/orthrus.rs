#![feature(proc_macro_hygiene, decl_macro, array_map)]
#![deny(
	missing_docs,
	missing_debug_implementations,
	missing_copy_implementations,
	trivial_casts,
	trivial_numeric_casts,
	unsafe_code,
	unused_import_braces,
	unused_qualifications
)]

//! Orthrus is an endpoint for Cerberus that sends Aeacus releases to an
//! authenticated client.

#[macro_use]
extern crate rocket;

use anyhow::Result;
use dotenv::dotenv;

mod orthrus;

fn main() -> Result<()> {
	// Load enviroment variables from .env
	dotenv().ok();

	// Run path structure check
	orthrus::check()?;

	// Initialize web server
	rocket::ignite().mount("/", routes![orthrus::root]).launch();

	Ok(())
}
