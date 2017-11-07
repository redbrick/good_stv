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

fn main() {
    println!("Hello, world!");
}
