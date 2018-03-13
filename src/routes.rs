use rocket::Request;
use rocket::response::NamedFile;
use rocket_contrib::Template;

use std::path::{Path, PathBuf};
use std::string::String;
use std::collections::HashMap;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/")]
pub fn root() -> Template {
    let context = TemplateContext {
        name: String::from("GOOD_STV"),
    };

    Template::render("index", &context)
}

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

#[error(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().as_str());
    Template::render("error/404", &map)
}
