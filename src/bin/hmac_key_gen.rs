// src/bin/hmac_key_gen.rs
use argon2::password_hash::rand_core::{OsRng, RngCore};

fn main() {
    // Generate 32 bytes (256 bits) of cryptographically secure random data
    let mut key = [0u8; 32];
    OsRng.fill_bytes(&mut key);

    // Encode as hex for storage in .env
    let hex_key = hex::encode(key);

    println!("HMAC Key (hex-encoded):");
    println!("{hex_key}");
    println!();
    println!("Add this to your .env file:");
    println!("SESSION_HMAC_KEY={hex_key}");
}
