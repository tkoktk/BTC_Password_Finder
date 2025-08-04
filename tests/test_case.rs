use btc_password_finder::mutations::apply_case_variations;


//
#[test]
fn test_basic_case_variations() {
    let result = apply_case_variations("password");
    
    println!("Case variations for 'password': {:?}", result);
    
    // Should contain the original and different cases
    assert!(result.contains(&"password".to_string()));
    assert!(result.contains(&"PASSWORD".to_string()));
    assert!(result.contains(&"Password".to_string()));
    assert!(result.contains(&"pAsSwOrD".to_string())); // alternating
    
    // Should not contain duplicates
    let unique_count = result.len();
    let mut deduped = result.clone();
    deduped.sort();
    deduped.dedup();
    assert_eq!(unique_count, deduped.len(), "Should not contain duplicates");
}

#[test]
fn test_single_character() {
    let result = apply_case_variations("a");
    
    println!("Case variations for 'a': {:?}", result);
    
    assert!(result.contains(&"a".to_string()));
    assert!(result.contains(&"A".to_string()));
    // Single character shouldn't have alternating case
}

#[test]
fn test_empty_string() {
    let result = apply_case_variations("");
    
    println!("Case variations for empty string: {:?}", result);
    
    assert_eq!(result.len(), 1);
    assert_eq!(result[0], "");
}

#[test]
fn test_already_uppercase() {
    let result = apply_case_variations("PASSWORD");
    
    println!("Case variations for 'PASSWORD': {:?}", result);
    
    assert!(result.contains(&"PASSWORD".to_string()));
    assert!(result.contains(&"password".to_string()));
    assert!(result.contains(&"Password".to_string()));
}

#[test]
fn test_mixed_case_input() {
    let result = apply_case_variations("PaSsWoRd");
    
    println!("Case variations for 'PaSsWoRd': {:?}", result);
    
    // Should include original
    assert!(result.contains(&"PaSsWoRd".to_string()));
    // And standard variations
    assert!(result.contains(&"password".to_string()));
    assert!(result.contains(&"PASSWORD".to_string()));
    assert!(result.contains(&"Password".to_string()));
}

#[test]
fn test_numbers_and_symbols() {
    let result = apply_case_variations("Pass123!");
    
    println!("Case variations for 'Pass123!': {:?}", result);
    
    assert!(result.contains(&"Pass123!".to_string()));
    assert!(result.contains(&"pass123!".to_string()));
    assert!(result.contains(&"PASS123!".to_string()));
    assert!(result.contains(&"pAsS123!".to_string())); // alternating
}