/* good_stv - a good single transferable vote utility.
 * Copyright (C) 2019 good_stv authors
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
#![warn(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_import_braces,
    unused_qualifications
)]

mod poll;
mod routes;
#[cfg(test)]
mod test;

use std::collections::HashMap;
use std::sync::RwLock;

use clap::App;
use failure::Error;
use rocket::{catchers, routes};

use routes::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(Debug)]
pub struct PollDb {
    polls: RwLock<HashMap<String, poll::Poll>>,
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .manage(PollDb {
            polls: RwLock::new(HashMap::new()),
        })
        .mount(
            "/",
            routes![get_poll, close_poll, create_poll, vote, get_results],
        )
        .register(catchers![not_found, internal_error])
}

fn main() -> Result<(), Error> {
    let _matches = App::new("good_stv_server")
        .version(VERSION.unwrap_or("unknown"))
        .author("Terry Bolt <tbolt@redbrick.dcu.ie>")
        .about("The good_stv web service.")
        .get_matches();

    Err(rocket().launch().into())
}
