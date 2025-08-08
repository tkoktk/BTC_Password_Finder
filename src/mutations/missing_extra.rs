/// Handle missing or extra characters
pub fn apply_missing_extra(word: &str) -> Vec<String> {
    let mut variations = Vec::new();
    variations.push(word.to_string());
    
    let chars: Vec<char> = word.chars().collect();
    
    // Missing characters (deletions)
    for i in 0..chars.len() {
        let mut deleted = chars.clone();
        deleted.remove(i);
        let variant = deleted.iter().collect::<String>();
        if !variations.contains(&variant) {
            variations.push(variant);
        }
    }
    
    // Common extra characters at positions
    let common_extras = vec!['1', '!', '@', '0', 's', 'S'];
    
    // Add at beginning
    for &extra in &common_extras {
        let variant = format!("{}{}", extra, word);
        if !variations.contains(&variant) {
            variations.push(variant);
        }
    }
    
    // Add at end
    for &extra in &common_extras {
        let variant = format!("{}{}", word, extra);
        if !variations.contains(&variant) {
            variations.push(variant);
        }
    }
    
    // Duplicate characters
    for i in 0..chars.len() {
        let mut doubled = Vec::new();
        for (j, &ch) in chars.iter().enumerate() {
            doubled.push(ch);
            if i == j {
                doubled.push(ch); // Double this character
            }
        }
        let variant = doubled.iter().collect::<String>();
        if !variations.contains(&variant) {
            variations.push(variant);
        }
    }
    
    variations
}