/// Common patterns in complex passwords
pub fn apply_pattern_mutations(word: &str) -> Vec<String> {
    let mut variations = Vec::new();
    variations.push(word.to_string());
    
    // Handle @ symbol variations
    if word.contains('@') {
        // Replace @ with 'at'
        variations.push(word.replace('@', "at"));
        variations.push(word.replace('@', "AT"));
        variations.push(word.replace('@', "At"));
        
        // Remove @ completely
        variations.push(word.replace('@', ""));
        
        // Double @
        variations.push(word.replace('@', "@@"));
    }
    
    // Handle potential 'at' that should be @
    if word.contains("at") {
        variations.push(word.replace("at", "@"));
    }
    if word.contains("AT") {
        variations.push(word.replace("AT", "@"));
    }
    if word.contains("At") {
        variations.push(word.replace("At", "@"));
    }
    
    // Handle BT pattern (might be a typo or abbreviation)
    if word.contains("BT") {
        variations.push(word.replace("BT", "bt"));
        variations.push(word.replace("BT", "Bt"));
        variations.push(word.replace("BT", "AT")); // Common typo
        variations.push(word.replace("BT", "BY")); // Adjacent keys
        variations.push(word.replace("BT", "B")); // Missing T
        variations.push(word.replace("BT", "T")); // Missing B
    }
    
    // Handle NMA pattern
    if word.contains("NMA") {
        variations.push(word.replace("NMA", "nma"));
        variations.push(word.replace("NMA", "MNA")); // Transposition
        variations.push(word.replace("NMA", "NAM")); // Transposition
        variations.push(word.replace("NMA", "NM")); // Missing A
        variations.push(word.replace("NMA", "MA")); // Missing N
    }
    
    // Handle ER pattern
    if word.contains("ER") {
        variations.push(word.replace("ER", "er"));
        variations.push(word.replace("ER", "Er"));
        variations.push(word.replace("ER", "RE")); // Transposition
    }
    
    variations
}