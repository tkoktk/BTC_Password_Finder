//Passwords should be generated in batches on demand for efficiency
//10-30 seconds of work per batch, adapt to mutation complexity

fn generate_password_batch(
    base_words: &[&str],
    target_work_seconds: f64,  // e.g., 30.0
    mutation_functions: Vec<fn(&str) -> Vec<String>>
) -> Vec<String> {
    let mut batch = Vec::new();

    // for word in base_words {
    //     let mut word_mutations = Vec::new();

    //     for mutation_fn in &mutation_functions {

    //     }
    //     //Apply mutations to word

    //     word_mutations;
    // }
    batch
}