mod config;
mod mutations;
mod engine;

use engine::Coordinator;


//use crate::config::{read_mutation_file, write_found_password, read_private_key_file};

fn main() {
    //RUSTFLAGS="-C target-cpu=native" cargo run --release

    //Murphy 6PRKpn2ftLhcgPcU9TdkgmaQtxZHttzHNfYth1r2p8RZxQP63QJ726p52r

//start here
    let mut coordinator = Coordinator::new();
    //JoHnNyApPlEs
    match coordinator.start() {
        Ok(()) => println!("Coordinator run completed successfully"),
        Err(e) => eprintln!("Error during cracking: {}", e),
    }
}
