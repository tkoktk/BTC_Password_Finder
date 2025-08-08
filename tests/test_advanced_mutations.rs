use btc_password_finder::mutations::AdvancedMutator;
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_comprehensive_mutations() {
        let mutator = AdvancedMutator::new();
        let mutations = mutator.generate_comprehensive_mutations("test@123");
        
        println!("Generated {} mutations for 'test@123'", mutations.len());
        for (i, mutation) in mutations.iter().take(20).enumerate() {
            println!("{}: {}", i + 1, mutation);
        }
        
        assert!(mutations.len() > 10);
        assert!(mutations.contains(&"test@123".to_string()));
    }