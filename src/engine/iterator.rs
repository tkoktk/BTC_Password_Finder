// src/engine/iterator.rs - Enhanced version
use crate::mutations::{apply_all_mutations, apply_case_variations};
use crate::config::read_password_ideas;
use std::collections::HashSet;

pub struct PasswordIterator {
    base_words: Vec<String>,
    current_word_index: usize,
    current_mutations: Vec<String>,
    current_mutation_index: usize,
    tried_passwords: HashSet<String>,
    mode: AttackMode,
}

#[derive(Debug, Clone)]
enum AttackMode {
    SmartMutations,    // Comprehensive mutations
    FullMutations,     // All mutations on base words
    Combinations,      // Try combining segments
}

impl PasswordIterator {
    pub fn new() -> Self {
        let base_words = read_password_ideas();
        
        println!("Loaded {} password ideas from file", base_words.len());
        
        if base_words.is_empty() {
            eprintln!("Error: No password ideas found in password_ideas.txt");
            eprintln!("Please add your password patterns to password_ideas.txt");
            std::process::exit(1);
        }
        
        // Start with comprehensive mutations on first password idea
        let current_mutations = apply_all_mutations(&base_words[0]);
        println!("Generated {} mutations for first pattern: '{}'", 
                 current_mutations.len(), base_words[0]);
        
        Self {
            base_words,
            current_word_index: 0,
            current_mutations,
            current_mutation_index: 0,
            tried_passwords: HashSet::new(),
            mode: AttackMode::SmartMutations,
        }
    }


    pub fn next_batch(&mut self, batch_size: usize) -> Option<Vec<String>> {
        let mut batch = Vec::new();
        
        while batch.len() < batch_size {
            if self.current_mutation_index < self.current_mutations.len() {
                let password = self.current_mutations[self.current_mutation_index].clone();
                self.current_mutation_index += 1;
                
                // Skip if we've already tried this password
                if !self.tried_passwords.contains(&password) {
                    self.tried_passwords.insert(password.clone());
                    batch.push(password);
                }
            } else {
                // Current word exhausted, move to next
                if !self.advance_to_next_word() {
                    // Try next attack mode
                    if !self.advance_attack_mode() {
                        break; // All modes exhausted
                    }
                }
            }
        }

        if batch.is_empty() {
            None
        } else {
            Some(batch)
        }
    }
    
    fn advance_to_next_word(&mut self) -> bool {
        self.current_word_index += 1;
        if self.current_word_index >= self.base_words.len() {
            return false;
        }

        // Apply mutations based on current mode
        self.current_mutations = match self.mode {
            AttackMode::SmartMutations | AttackMode::FullMutations => {
                apply_all_mutations(&self.base_words[self.current_word_index])
            },
            AttackMode::Combinations => {
                // For combinations mode, try segment combinations
                self.generate_segment_combinations(&self.base_words[self.current_word_index])
            },
        };
        
        self.current_mutation_index = 0;
        
        // Advanced to next word (silent)

        true
    }
    fn advance_attack_mode(&mut self) -> bool {
        self.mode = match self.mode {
            AttackMode::SmartMutations => {
                println!("🔄 Entering comprehensive mutation mode");
                AttackMode::FullMutations
            },
            AttackMode::FullMutations => {
                println!("🔄 Entering combination attack mode");
                AttackMode::Combinations
            },
            AttackMode::Combinations => {
                return false; // No more modes
            }
        };
        
        self.current_word_index = 0;
        if !self.base_words.is_empty() {
            self.current_mutations = apply_all_mutations(&self.base_words[0]);
            self.current_mutation_index = 0;
        }
        
        true
    }
    
    fn generate_segment_combinations(&self, base: &str) -> Vec<String> {
        let mut combinations = Vec::new();
        combinations.push(base.to_string());
        
        // Try splitting on common delimiters and recombining
        let delimiters = vec!['@', '_', '-', '.', '!'];
        
        for delim in &delimiters {
            if base.contains(*delim) {
                let parts: Vec<&str> = base.split(*delim).collect();
                
                // Recombine with different delimiters
                for alt_delim in &delimiters {
                    let recombined = parts.join(&alt_delim.to_string());
                    if !combinations.contains(&recombined) {
                        combinations.push(recombined);
                    }
                }
                
                // Recombine without delimiter
                let joined = parts.join("");
                if !combinations.contains(&joined) {
                    combinations.push(joined);
                }
            }
        }
        
        // Apply case variations to each combination
        let mut all_combinations = Vec::new();
        for combo in combinations {
            for variant in apply_case_variations(&combo) {
                if !all_combinations.contains(&variant) {
                    all_combinations.push(variant);
                }
            }
        }
        
        all_combinations
    }
}