use std::collections::HashMap;

/// Common character substitutions that might occur in typing
pub fn get_substitution_map() -> HashMap<char, Vec<char>> {
    let mut map = HashMap::new();
    
    // Similar looking characters
    map.insert('@', vec!['a', 'A', '2', '4']);
    map.insert('a', vec!['@', '4', 'A']);
    map.insert('A', vec!['@', '4', 'a']);
    map.insert('e', vec!['3', 'E']);
    map.insert('E', vec!['3', 'e']);
    map.insert('i', vec!['1', '!', 'I', 'l']);
    map.insert('I', vec!['1', '!', 'i', 'l']);
    map.insert('l', vec!['1', '!', 'i', 'I']);
    map.insert('o', vec!['0', 'O']);
    map.insert('O', vec!['0', 'o']);
    map.insert('s', vec!['5', '$', 'S']);
    map.insert('S', vec!['5', '$', 's']);
    map.insert('t', vec!['7', 'T', '+']);
    map.insert('T', vec!['7', 't', '+']);
    map.insert('z', vec!['2', 'Z']);
    map.insert('Z', vec!['2', 'z']);
    map.insert('b', vec!['6', 'B']);
    map.insert('B', vec!['6', 'b']);
    map.insert('g', vec!['9', 'G']);
    map.insert('G', vec!['9', 'g']);
    
    // Number substitutions
    map.insert('0', vec!['o', 'O']);
    map.insert('1', vec!['i', 'I', 'l', '!']);
    map.insert('2', vec!['z', 'Z', '@']);
    map.insert('3', vec!['e', 'E']);
    map.insert('4', vec!['a', 'A', '@']);
    map.insert('5', vec!['s', 'S', '$']);
    map.insert('6', vec!['b', 'B']);
    map.insert('7', vec!['t', 'T']);
    map.insert('8', vec!['B']);
    map.insert('9', vec!['g', 'G']);
    
    map
}

/// Apply character substitutions to generate variations
pub fn apply_substitutions(word: &str, max_substitutions: usize) -> Vec<String> {
    let mut variations = Vec::new();
    variations.push(word.to_string());
    
    let sub_map = get_substitution_map();
    let chars: Vec<char> = word.chars().collect();
    
    // Single substitutions
    for (i, &ch) in chars.iter().enumerate() {
        if let Some(substitutes) = sub_map.get(&ch) {
            for &sub in substitutes {
                let mut new_word = chars.clone();
                new_word[i] = sub;
                let variant = new_word.iter().collect::<String>();
                if !variations.contains(&variant) {
                    variations.push(variant);
                }
            }
        }
    }
    
    // Double substitutions if requested
    if max_substitutions >= 2 && chars.len() >= 2 {
        for i in 0..chars.len() {
            for j in (i+1)..chars.len() {
                if let (Some(subs_i), Some(subs_j)) = (sub_map.get(&chars[i]), sub_map.get(&chars[j])) {
                    for &sub_i in subs_i {
                        for &sub_j in subs_j {
                            let mut new_word = chars.clone();
                            new_word[i] = sub_i;
                            new_word[j] = sub_j;
                            let variant = new_word.iter().collect::<String>();
                            if !variations.contains(&variant) {
                                variations.push(variant);
                            }
                        }
                    }
                }
            }
        }
    }
    
    variations
}