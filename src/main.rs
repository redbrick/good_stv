extern crate csv;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;

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
