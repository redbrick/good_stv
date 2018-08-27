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

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate clap;
extern crate env_logger;
extern crate failure;
extern crate good_stv;
extern crate log;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod routes;

use clap::App;
use failure::*;
use rocket_contrib::Template;

use routes::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    if let Err(err) = run() {
        log::debug!("{:?}", err);
        log::error!("{}", err);
        for cause in err.iter_chain().skip(1) {
            log::error!("Caused by: {}", cause);
        }
        std::process::exit(1);
    }
}

fn run() -> Result<(), Error> {
    env_logger::init();

    let _matches = App::new("good_stv_server")
        .version(VERSION.unwrap_or("unknown"))
        .author("Terry Bolt <tbolt@redbrick.dcu.ie>")
        .about("The good_stv web service.")
        .get_matches();

    rocket::ignite()
        .mount("/", routes![root, files])
        .attach(Template::fairing())
        .catch(catchers![not_found])
        .launch();

    Ok(())
}
