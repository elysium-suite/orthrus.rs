#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[get("/?<os>&<pass>")]
fn index(os: Option<String>, pass: Option<String>) -> String {
    match os {
        Some(_) => match pass {
            Some(r) => format!("Pass: {}", r),
            None => format!("No pass!"),
        },
        None => format!("No value!"),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
