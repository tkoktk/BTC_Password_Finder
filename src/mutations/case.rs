/// Returns a vector of case variations
pub fn apply_case_variations(word: &str) -> Vec<String> {
    let mut variations = Vec::new();
    
    // Original case
    variations.push(word.to_string());

    let lowercase = word.to_lowercase();
    let add_lowercase_to_word = lowercase != word;
    
    let uppercase = word.to_uppercase();
    let add_uppercase_to_word = uppercase != word;

        let title_case = if !word.is_empty() {
        Some(format!("{}{}",
            word.chars().next().unwrap().to_uppercase(),
            word.chars().skip(1).collect::<String>().to_lowercase()
        ))
    } else {
        None
    };

    let should_add_title_case = if let Some(ref tc) = title_case {
        tc != word && tc != &lowercase && tc != &uppercase
    } else {
        false
    };

    //After all comparisons, push. Means we don't have to clone anything

    if add_lowercase_to_word {
        variations.push(lowercase);
    }

    if add_uppercase_to_word {
        variations.push(uppercase);
    }

    if should_add_title_case {
        variations.push(title_case.unwrap());
    }            

    if word.len() > 1 {
        let alternating: String = word.chars()
            .enumerate()
            .map(|(i, c)| {
                if i % 2 == 0 {
                    c.to_lowercase().to_string()
                } else {
                    c.to_uppercase().to_string()
                }
            })
            .collect();
        
        if alternating != word && !variations.contains(&alternating) {
            variations.push(alternating);
        }
    }                                                                                                      

    variations
}