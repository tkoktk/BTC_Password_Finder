/// Split password into segments and apply mutations to each
pub fn segment_mutations(password: &str) -> Vec<String> {
    let mut variations = Vec::new();
    variations.push(password.to_string());
    
    // Common segment delimiters in your pattern
    let delimiters = vec!['@', '2', '3', '6', '8'];
    
    // Try different segmentations
    for delimiter in &delimiters {
        if password.contains(*delimiter) {
            let parts: Vec<&str> = password.split(*delimiter).collect();
            
            // Rejoin with variations
            if parts.len() > 1 {
                // Missing delimiter
                variations.push(parts.join(""));
                
                // Double delimiter
                let double_delim = format!("{}{}", delimiter, delimiter);
                variations.push(parts.join(&double_delim));
                
                // Different delimiter
                for alt_delim in &delimiters {
                    if *alt_delim != *delimiter {
                        variations.push(parts.join(&alt_delim.to_string()));
                    }
                }
            }
        }
    }
    
    variations
}