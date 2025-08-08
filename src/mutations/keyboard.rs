/// QWERTY keyboard layout for proximity errors
const KEYBOARD: &[&str] = &[
    "1234567890",
    "qwertyuiop",
    "asdfghjkl",
    "zxcvbnm"
];

/// Get adjacent keys on QWERTY keyboard
pub fn get_adjacent_keys(ch: char) -> Vec<char> {
    let mut adjacent = Vec::new();
    let lower = ch.to_ascii_lowercase();
    
    for (row_idx, row) in KEYBOARD.iter().enumerate() {
        if let Some(col_idx) = row.find(lower) {
            // Left neighbor
            if col_idx > 0 {
                adjacent.push(row.chars().nth(col_idx - 1).unwrap());
            }
            // Right neighbor
            if col_idx < row.len() - 1 {
                adjacent.push(row.chars().nth(col_idx + 1).unwrap());
            }
            // Upper neighbor
            if row_idx > 0 {
                if let Some(upper_char) = KEYBOARD[row_idx - 1].chars().nth(col_idx) {
                    adjacent.push(upper_char);
                }
            }
            // Lower neighbor
            if row_idx < KEYBOARD.len() - 1 {
                if let Some(lower_char) = KEYBOARD[row_idx + 1].chars().nth(col_idx) {
                    adjacent.push(lower_char);
                }
            }
        }
    }
    
    // Maintain case if original was uppercase
    if ch.is_uppercase() {
        adjacent.iter().map(|&c| c.to_ascii_uppercase()).collect()
    } else {
        adjacent
    }
}

/// Apply keyboard proximity errors
pub fn apply_keyboard_errors(word: &str) -> Vec<String> {
    let mut variations = Vec::new();
    variations.push(word.to_string());
    
    let chars: Vec<char> = word.chars().collect();
    
    for (i, &ch) in chars.iter().enumerate() {
        let adjacent = get_adjacent_keys(ch);
        for adj in adjacent {
            let mut new_word = chars.clone();
            new_word[i] = adj;
            let variant = new_word.iter().collect::<String>();
            if !variations.contains(&variant) {
                variations.push(variant);
            }
        }
    }
    
    variations
}