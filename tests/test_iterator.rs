use btc_password_finder::engine::PasswordIterator;

#[test]
fn test_password_iterator_basic() {
    let mut iterator = PasswordIterator::new();
    
    let mut total_passwords = 0;
    while let Some(batch) = iterator.next_batch(8) {
        println!("Batch: {:?}", batch);
        total_passwords += batch.len();
        if total_passwords > 100 {
            break;
        }
    }
    
    println!("Total passwords generated: {}", total_passwords);
}