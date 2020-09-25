#![feature(proc_macro_hygiene, decl_macro)]
use dotenv::dotenv;
use rocket::response::NamedFile;

use std::env;
use std::io;
use std::path::Path;

#[macro_use]
extern crate rocket;

// Create custom error enum to
// wrap possible returns
#[derive(Debug)]
enum Error {
    Env(std::env::VarError),
    Io(std::io::Error),
}

// Make sure directory structure is compliant
fn check() -> bool {
    if Path::new(".env").exists()
        && Path::new("dist/aeacus-win32.zip").exists()
        && Path::new("dist/aeacus-linux.zip").exists()
    {
        return true;
    }
    false
}

// Main route for server
#[get("/?<os>&<pass>")]
fn index(os: String, pass: String) -> Result<NamedFile, Error> {
    // Read KEY environment variable into string
    // and fail if it's not present
    let key = env::var("KEY").map_err(Error::Env)?;

    // If the password equals the key, return
    // the appropriate file
    if pass == key {
        NamedFile::open(Path::new("dist").join(format!("aeacus-{}.zip", os))).map_err(Error::Io)
    } else {
        Err(Error::Io(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "Wrong pass!",
        )))
    }
}

fn main() {
    // Load enviroment variables
    dotenv().ok();

    // Run path structure check
    if !check() {
        panic!("Required files do not exist!");
    }

    // Initialize web server
    rocket::ignite().mount("/", routes![index]).launch();
}
