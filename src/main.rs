#![feature(proc_macro_hygiene, decl_macro)]
use dotenv::dotenv;
use rocket::http;
use rocket::request::Request;
use rocket::response::{self, NamedFile, Responder, Response};
use serde::Serialize;

use std::env;
use std::io;
use std::path::Path;

#[macro_use]
extern crate rocket;

#[derive(Debug, Serialize)]
struct PassErr {
    message: String,
    tried_pass: String,
}

impl PassErr {
    fn new(m: &str, t: &str) -> PassErr {
        PassErr {
            message: m.into(),
            tried_pass: t.into(),
        }
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        if let Error::Pass(pass_err) = self {
            let res = PassErr::new(&pass_err.message, &pass_err.tried_pass);
            return Response::build()
                .sized_body(io::Cursor::new(serde_json::to_string(&res).unwrap()))
                .raw_header("Content-Type", "application/json")
                .ok();
        }
        Err(http::Status::new(404, "Not Found"))
    }
}

// Create custom error enum to
// wrap possible returns
#[derive(Debug)]
enum Error {
    Env(std::env::VarError),
    Io(io::Error),
    Pass(PassErr),
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
        Err(PassErr::new("Wrong password!", &pass)).map_err(Error::Pass)
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
