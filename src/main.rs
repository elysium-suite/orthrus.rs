#![feature(proc_macro_hygiene, decl_macro)]
use rocket::response::NamedFile;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

#[macro_use]
extern crate rocket;

fn return_file(file_name: String, pass: Option<String>) -> Result<NamedFile, Error> {
    match pass {
        Some(p) => {
            if p == "bruh".to_string() {
                let f =
                    NamedFile::open(Path::new("static/").join(format!("aeacus-{}.zip", file_name)));

                match f {
                    Ok(f) => Ok(f),
                    Err(err) => Err(err),
                }
            } else {
                Err(Error::new(ErrorKind::PermissionDenied, "Wrong pass!"))
            }
        }
        None => Err(Error::new(ErrorKind::PermissionDenied, "No pass!")),
    }
}

#[get("/?<os>&<pass>")]
fn index(os: Option<String>, pass: Option<String>) -> Result<NamedFile, String> {
    match os {
        Some(o) => match o.as_str() {
            "win32" => Ok(return_file("win32".to_string(), pass).unwrap()),
            "linux" => Ok(return_file("win32".to_string(), pass).unwrap()),
            _ => Err(Error::new(ErrorKind::InvalidInput, "Invalid OS!")),
        },
        None => Err(Error::new(ErrorKind::InvalidInput, "No OS!")),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
