use std::env;
use std::path::Path;
use std::process;
use std::fs;

pub fn get_mutation_file() -> String {
    
    let file_path = env::var("MUTATIONS_FILE")
        .unwrap_or("default_mutations".to_string()) + ".txt";
    
     if Path::new(&file_path).exists() {
        println!("Using mutation file: {}", file_path);
        file_path
    } else {
        eprintln!("Error: Mutation file '{}' not found", file_path);
        process::exit(1);
    }
}

pub fn read_mutation_file() -> Vec<String> {
    let file_path = get_mutation_file();
    
    match fs::read_to_string(&file_path) {
        Ok(contents) => {
            // Split by lines and clean up
            contents
                .lines()
                .map(|line| line.trim().to_string())
                .filter(|line| !line.is_empty())    
                .collect()
        }
        Err(error) => {
            eprintln!("Error reading mutation file '{}': {}", file_path, error);
            process::exit(1);
        }
    }
}