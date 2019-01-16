/* good_stv - a good single transferable vote utility.
 * Copyright (C) 2017 Terry Bolt
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::string::String;

use rocket::response::NamedFile;
use rocket::Request;
use log::*;
use rocket::{catch, get};
use rocket_contrib::templates::Template;
use serde_derive::Serialize;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
}

#[get("/")]
pub fn root() -> Template {
    let context = TemplateContext {
        name: String::from("GOOD_STV"),
    };
    error!("test");

    Template::render("index", &context)
}

#[get("/<file..>")]
pub fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("public/").join(file)).ok()
}

#[catch(404)]
pub fn not_found(req: &Request) -> Template {
    let mut map = HashMap::<String, String>::new();
    map.insert("path".to_string(), req.uri().to_string());
    Template::render("error/404", &map)
}
