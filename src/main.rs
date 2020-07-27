#![feature(proc_macro_hygiene, decl_macro)]
use rocket::response::NamedFile;
use std::fs;
use std::io::Error;
use std::io::ErrorKind;
use std::path::Path;

#[macro_use]
extern crate rocket;

fn return_file(file_name: String, pass: Option<String>) -> Result<NamedFile, Error> {
    let auth = fs::read_to_string("pass.txt");
    let auth = match auth {
        Ok(o) => o,
        Err(_) => "default_secure_password".to_string(),
    };

    match pass {
        Some(p) => {
            if p == auth {
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
            "win32" => {
                let res = return_file("win32".to_string(), pass);
                match res {
                    Ok(r) => Ok(r),
                    Err(e) => Err(format!("{}", e)),
                }
            }
            "linux" => {
                let res = return_file("linux".to_string(), pass);
                match res {
                    Ok(r) => Ok(r),
                    Err(e) => Err(format!("{}", e)),
                }
            }
            _ => Err(format!("Invalid OS!")),
        },
        None => Err(format!("No OS!")),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
