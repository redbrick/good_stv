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

#![feature(proc_macro_hygiene, decl_macro)]

mod routes;

use clap::App;
use failure::Error;
use rocket::{catchers, routes};
use rocket_contrib::templates::Template;

use routes::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() -> Result<(), Error> {
    let _matches = App::new("good_stv_server")
        .version(VERSION.unwrap_or("unknown"))
        .author("Terry Bolt <tbolt@redbrick.dcu.ie>")
        .about("The good_stv web service.")
        .get_matches();

    Err(rocket::ignite()
        .mount("/", routes![root, files])
        .attach(Template::fairing())
        .register(catchers![not_found])
        .launch().into())
}
