use std::fs::File;
use std::io::{BufWriter, Write};
use crate::mutations::apply_case_variations;
use crate::config::{get_config_file, read_password_ideas};

pub struct PasswordGenerator {
    base_words: Vec<String>,
    output_file: BufWriter<File>,
}

impl PasswordGenerator {

    pub fn new() -> Result<Self, std::io::Error> {
        //1: Read in user's password ideas
        let base_words = read_password_ideas();
        println!("Loaded the following password ideas: {}", base_words.len());
        
        //2: Set up output file for mutations
        let output_path = get_config_file("MUTATIONS", "default_mutations");
        let file = File::create(&output_path)?;
        let output_file = BufWriter::new(file);
        println!("Writing mutations to: {}", output_path);
        
        Ok(PasswordGenerator { base_words, output_file })
    }
    
    // pub fn generate_all_mutations(&mut self) -> Result<usize, std::io::Error> {
    //     // Loop through each base word
    //     // Apply mutations 
    //     // Write to file
    //     // Return total password count
    // }
}