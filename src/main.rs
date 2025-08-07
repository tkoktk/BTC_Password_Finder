mod config;
use btc_password_finder::config::read_mutation_file;

use crate::config::get_mutation_file;
fn main() {
    let cpu_cores = std::thread::available_parallelism().unwrap().get();
    println!("Hello, world!, {}", cpu_cores); 

    let mutations = get_mutation_file();
    println!("Hello, world!, {}", mutations);

    let base_words = read_mutation_file();
    println!("Loaded {} base words: {:?}", base_words.len(), base_words);
}
