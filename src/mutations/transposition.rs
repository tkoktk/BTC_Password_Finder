/// Apply character transpositions (swapped adjacent characters)
pub fn apply_transpositions(word: &str) -> Vec<String> {
    let mut variations = Vec::new();
    variations.push(word.to_string());
    
    let chars: Vec<char> = word.chars().collect();
    
    // Swap adjacent characters
    for i in 0..chars.len().saturating_sub(1) {
        let mut swapped = chars.clone();
        swapped.swap(i, i + 1);
        let variant = swapped.iter().collect::<String>();
        if !variations.contains(&variant) {
            variations.push(variant);
        }
    }
    
    variations
}