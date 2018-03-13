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
#![feature(custom_attribute)]
#![feature(use_extern_macros)]

extern crate clap;
extern crate csv;
extern crate env_logger;
#[macro_use]
extern crate failure;
#[macro_use(log)]
extern crate log;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod routes;
mod stv;

use std::io;

use clap::{App, Arg, ArgGroup};
use rocket_contrib::Template;

use failure::{Error, ResultExt};
use routes::*;
use stv::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

fn main() {
    use std::process::exit;

    if let Err(err) = run() {
        log::debug!("{:?}", err);
        eprintln!("{}", err);
        for cause in err.causes().skip(1) {
            eprintln!("Caused by: {}", cause);
        }
        exit(1);
    }
}

fn run() -> Result<(), Error> {
    env_logger::init();

    let matches = App::new("good_stv")
        .version(VERSION.unwrap_or("unknown"))
        .author("Terry Bolt <tbolt@redbrick.dcu.ie>")
        .about("A tool for evaluating elections using Single Transferable Vote.")
        .arg(
            Arg::with_name("seats")
                .index(1)
                .help("Number of seats to be filled."),
        )
        .arg(
            Arg::with_name("server")
                .short("S")
                .long("server")
                .help("Run server for web interface"),
        )
        .arg(
            Arg::with_name("file")
                .short("f")
                .long("file")
                .value_name("FILE")
                .help("CSV file to read votes from.")
                .long_help(
                    "The CSV file must be in the following format:

candidate_name,candidate_name,candidate_name,...
first_preference_candidate,second_preference_candidate,...
first_preference_candidate,second_preference_candidate,...
...",
                ),
        )
        .group(
            ArgGroup::with_name("vers")
                .args(&["seats", "server"])
                .required(true),
        )
        .get_matches();

    if matches.is_present("server") {
        rocket::ignite()
            .mount("/", routes![root, files])
            .attach(Template::fairing())
            .catch(errors![not_found])
            .launch();
    } else {
        let seats: u64 = matches
            .value_of("seats")
            .unwrap()
            .parse::<u64>()
            .context("Invalid input for seats. Must be an integer.")?;
        let election = if matches.is_present("file") {
            Election::from_csv_file(matches.value_of("file").unwrap(), seats)?
        } else {
            Election::from_reader(io::stdin(), seats)?
        };

        let results = election.results()?;

        print_results(&results);
    }

    Ok(())
}

fn print_results(results: &ElectionResults) {
    println!("Elected:");
    for elected in &results.elected {
        println!("\t{} with {} votes.", elected.0, elected.1);
    }
    println!("\nEliminated:");
    for eliminated in &results.eliminated {
        println!("\t{} with {} votes.", eliminated.0, eliminated.1);
    }
}
