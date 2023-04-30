#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket::response::NamedFile;

use rocket_contrib::serve::StaticFiles;

use reform;

fn main() {
    reform::build_rocket()
        .mount("/", StaticFiles::from("static"))
        .register(catchers![catch_all])
        .launch();
}

// Serve index.html for every route
#[catch(404)]
fn catch_all() -> NamedFile {
    NamedFile::open("static/index.html").expect("No index.html supplied")
}
