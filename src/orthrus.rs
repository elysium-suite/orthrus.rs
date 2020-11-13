use anyhow::{anyhow, Context, Result};
use rocket::response::NamedFile;

use std::{env, path::Path};

// Make sure directory structure is compliant
pub(crate) fn check() -> Result<()> {
	if [".env", "dist/aeacus-win32.zip", "dist/aeacus-linux.zip"]
		.iter()
		.map(Path::new)
		.all(Path::exists)
	{
		return Ok(());
	}
	Err(anyhow!("Required files do not exist!"))
}

// Main route for server
#[get("/?<os>&<pass>")]
pub(crate) fn root(os: String, pass: String) -> Result<NamedFile> {
	// Read KEY environment variable into string
	// and fail if it's not present
	let key =
		env::var("KEY").context("Could not find enviroment variable: KEY")?;

	// If the password equals the key, return
	// the appropriate file
	if pass == key {
		let file = format!("aeacus-{}.zip", os);
		NamedFile::open(Path::new("dist").join(file))
			.context("Failed to open file!".to_string())
	} else {
		Err(anyhow!("Incorrect password!"))
	}
}
