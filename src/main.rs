use bip39::{Mnemonic, Language};

fn main() {
    // Generate a random mnemonic
    let mnemonic = Mnemonic::generate_in(Language::English, 24).unwrap();
    
    // Print the mnemonic phrase
    println!("Generated mnemonic: {}", mnemonic.to_string());

    // Optionally, you can also create a seed from the mnemonic

    // Pass an empty string as the passphrase
    let seed = mnemonic.to_seed(""); // Passphrase can be an empty string
    let seed_hex = hex::encode(seed);

    println!("Seed (hex): {}", seed_hex);
}
