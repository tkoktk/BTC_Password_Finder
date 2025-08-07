mod config;

use crate::config::{read_mutation_file,get_mutation_file, write_found_password};

fn main() {
    let cpu_cores = std::thread::available_parallelism().unwrap().get();
    println!("Hello, world!, {}", cpu_cores); 

    let mutations = get_mutation_file();
    println!("Hello, world!, {}", mutations);

    let base_words = read_mutation_file();
    println!("Loaded {} base words: {:?}", base_words.len(), base_words);

    write_found_password("haaaaNNNEEE").expect("Failed to write password");
}
