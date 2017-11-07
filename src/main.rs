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

extern crate clap;
extern crate csv;
extern crate env_logger;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate rand;

mod errors {
    error_chain!{
        foreign_links {
            Io(::std::io::Error);
        }
    }
}

mod stv;

use std::io;

use clap::{App, Arg};

use errors::*;
use stv::*;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

quick_main!(run);

fn run() -> Result<()> {
    env_logger::init().chain_err(
        || "Error initialising env_logger.",
    )?;

    let matches = App::new("good_stv")
        .version(VERSION.unwrap_or("unknown"))
        .author("Terry Bolt <tbolt@redbrick.dcu.ie>")
        .about(
            "A good single transferable vote utility for simple elections.",
        )
        .arg(Arg::with_name("seats").index(1).required(true).help(
            "Number of seats to be filled.",
        ))
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
        .get_matches();

    let seats: u64 = matches.value_of("seats").unwrap().parse().chain_err(
        || "Invalid input for seats.",
    )?;
    let election = if matches.is_present("file") {
        Election::from_csv_file(matches.value_of("file").unwrap(), seats)?
    } else {
        Election::from_reader(io::stdin(), seats)?
    };

    let results = election.results().chain_err(
        || "Error determining results.",
    )?;

    print_results(&results);

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
