use std::env;
use std::path::Path;
use std::process;
use std::fs;

///Abstracted code, try and read value from env var and if it doesn't exist use default txt file
pub fn get_config_file(env_var: &str, default_name: &str) -> String {
    let file_path = env::var(env_var)
        .unwrap_or(default_name.to_string()) + ".txt";
    
    if Path::new(&file_path).exists() {
        println!("Using {} file: {}", env_var.to_lowercase(), file_path);
        file_path
    } else {
        eprintln!("Error: {} file '{}' not found", env_var.to_lowercase(), file_path);
        process::exit(1);
    }
}

pub fn read_file_lines(env_var: &str, default_name: &str) -> Vec<String> {
    let file_path = get_config_file(env_var, default_name);
    
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

///Slightly different reading: we should not filter out spaces for passwords, and use line breaks as delimiters
pub fn read_password_ideas() -> Vec<String> {
    let file_path = get_config_file("PASSWORD_IDEAS", "password_ideas");
    
    match fs::read_to_string(&file_path) {
        Ok(contents) => {
            contents
                .lines()
                .map(|line| line.to_string())           // Keep original line as-is
                .filter(|line| !line.trim().is_empty()) // Only filter truly empty lines
                .collect()
        }
        Err(error) => {
            eprintln!("Error reading password ideas file '{}': {}", file_path, error);
            process::exit(1);
        }
    }
}

pub fn _read_mutation_file() -> Vec<String> {
    read_file_lines("MUTATIONS", "default_mutations")
}

pub fn read_private_key_file() -> String {
    let lines = read_file_lines("PRIVATE_KEY", "private_key");
    //We just want a single private key, so we'll just grab the first string
     match lines.len() {
        0 => {
            eprintln!("Error: Private key file is empty");
            process::exit(1);
        }
        1 => lines[0].clone(),
        n => {
            eprintln!("Error: Private key file should contain exactly one key, found {}", n);
            process::exit(1);
        }
    }
}

pub fn write_found_password(password: &str) -> Result<(), std::io::Error> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let content = format!("SUCCESS!\nPassword: {}\nTimestamp: {}\n", password, timestamp);
    
    fs::write("found_password.txt", content)?;
    println!("Found password written to: found_password.txt");
    Ok(())
}