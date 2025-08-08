pub mod case;
pub mod substitutions;
pub mod keyboard;
pub mod patterns;
pub mod segments;
pub mod transposition;
pub mod missing_extra;

pub use case::*;
pub use substitutions::*;
pub use keyboard::*;
pub use patterns::*;
pub use segments::*;
pub use transposition::*;
pub use missing_extra::*;

use std::collections::HashSet;

/// Master mutation function that applies all mutation strategies
pub fn apply_all_mutations(password: &str) -> Vec<String> {
    let mut all_variations = HashSet::new();
    
    // Add original
    all_variations.insert(password.to_string());
    
    // Level 1: Individual mutation types
    let mutations = vec![
        apply_case_variations(password),
        apply_substitutions(password, 2),
        apply_keyboard_errors(password),
        apply_pattern_mutations(password),
        apply_transpositions(password),
        apply_missing_extra(password),
        segment_mutations(password),
    ];
    
    for mutation_set in mutations {
        for variant in mutation_set {
            all_variations.insert(variant.clone());
            
            // Level 2: Apply case variations to each mutation
            for case_var in apply_case_variations(&variant) {
                all_variations.insert(case_var);
            }
        }
    }
    
    // Level 3: Combined mutations (be selective to avoid explosion)
    let current_variations: Vec<String> = all_variations.iter().cloned().collect();
    for variant in current_variations.iter().take(100) { // Limit to prevent explosion
        // Apply substitutions to transposed versions
        if variant.len() < 50 { // Reasonable length check
            for trans in apply_transpositions(variant).iter().take(5) {
                for sub in apply_substitutions(trans, 1).iter().take(5) {
                    all_variations.insert(sub.clone());
                }
            }
        }
    }
    
    all_variations.into_iter().collect()
}