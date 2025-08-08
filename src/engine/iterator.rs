use crate::mutations::apply_case_variations;
use crate::config::read_password_ideas;

pub struct PasswordIterator {
    base_words: Vec<String>,
    current_word_index: usize,
    current_mutations: Vec<String>,
    current_mutation_index: usize,
}

impl PasswordIterator {
    pub fn new() -> Self {

        let base_words = read_password_ideas();
        println!("Loaded {} password ideas", base_words.len());
        
        if base_words.is_empty() {
            eprintln!("Error: No password ideas found in file");
            std::process::exit(1);
        }
        
        //Will have a function later that expands this
        let current_mutations = apply_case_variations(&base_words[0]);
        println!("Generated {} mutations for first word: '{}'", current_mutations.len(), base_words[0]);
        
        Self {
            base_words,
            current_word_index: 0,
            current_mutations,
            current_mutation_index: 0,
        }
    }

    pub fn next_batch(&mut self, batch_size: usize) -> Option<Vec<String>> {
        let mut batch = Vec::new();
        
        while batch.len() < batch_size {
            if self.current_mutation_index < self.current_mutations.len() {
                batch.push(self.current_mutations[self.current_mutation_index].clone());
                self.current_mutation_index += 1
            } else {
                // Current word exhausted, move to next word
                if !self.advance_to_next_word() {
                    // No more words left
                    break;
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

        self.current_mutations = apply_case_variations(&self.base_words[self.current_word_index]);
        self.current_mutation_index = 0;

        true
    }
}