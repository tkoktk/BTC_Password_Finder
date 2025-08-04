use bip38::Decrypt;
// Grabbed these examples from the bip38 1.1.1 crate docs

#[test]
fn test_bip38_decryption_compressed() {
    let private_key = "6PYMgbeR64ypE4g8ZQhGo7ScudV5BLz1vMFUCs49AWpW3jVNWfH6cAdTi2";
    let password = "strong_pass";
    
    let result = private_key.decrypt(password);
    assert!(result.is_ok(), "Should successfully decrypt with correct password");
    
    let (private_key_bytes, compressed) = result.unwrap();
    assert_eq!(private_key_bytes, [0x11; 32], "Should return expected private key bytes");
    assert_eq!(compressed, true, "Should indicate compressed key");
    
    println!("Successfully decrypted compressed key: {:?}", private_key_bytes);
}

#[test]
fn test_bip38_decryption_with_generated_example() {
    //Manually generated a new wallet with the password "baysage"
    let private_key = "6PnMx7BZDf5xUWoPUDZaD8rbVJ5qtn95WaGuijZpwFgDuVJTTakyTvY2t9";
    let password = "baysage";

    let result = private_key.decrypt(password);
    assert!(result.is_ok(), "Should successfully decrypt with correct password");
}

#[test]
fn test_bip38_decryption_with_incorrect_password() {
    //Manually generated a new wallet with the password "baysage"
    //Try decrypt with correct private key but wrong password
    let private_key = "6PnMx7BZDf5xUWoPUDZaD8rbVJ5qtn95WaGuijZpwFgDuVJTTakyTvY2t9";
    let password = "password";

    let result = private_key.decrypt(password);
    assert!(result.is_err(), "Should not decrypt");
}

#[test]
fn test_bip38_decryption_with_incorrect_private_key() {
    //Manually generated a new wallet with the password "baysage"
    //Try decrypt with wrong priv key but right password
    let private_key = "6PnMx7BZDf5xUWoPUDZaD8rbVJ5qtn95WaGuijZpwFgDuVJTTaky";
    let password = "baysage";

    let result = private_key.decrypt(password);
    assert!(result.is_err(), "Should not decrypt");
}

#[test]
fn test_bip38_decryption_with_incorrect_password_space() {
    //Manually generated a new wallet with the password "baysage"
    //Try decrypt with correct private key but wrong password (has a space)
    let private_key = "6PnMx7BZDf5xUWoPUDZaD8rbVJ5qtn95WaGuijZpwFgDuVJTTakyTvY2t9";
    let password = "baysage ";

    let result = private_key.decrypt(password);
    assert!(result.is_err(), "Should not decrypt");
}
