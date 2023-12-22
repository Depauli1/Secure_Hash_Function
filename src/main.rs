/**
 * @title Applied Cryptography
 * @author Bless Hukporti
 * @notice Impliments Hash Function Implimentation that meets the requirements of collision resistance and avalanche effect.  
 * @dev Implementation of a secure hash function in Rust that uses the SHA-256 algorithm
 */
use sha2::{Digest, Sha256};

fn secure_hash(input: &str) -> String {
    // This creates a new Sha256 hasher
    let mut hasher = Sha256::new();

    hasher.update(input.as_bytes());

    let result = hasher.finalize();
    let hash_hex = format!("{:x}", result);

    hash_hex
}

fn main() {
    let input_data = "Bless, Hukporti";
    let hashed_result = secure_hash(input_data);
    println!("input: {}", input_data);
    println!("Hashes_Result: {}", hashed_result);
}

/*
input: hash, algorithm
Hashes_Result: b71505e25a00aa3287d76ef0417b02c1dc289b0e214fac78d85e683aa170e4f4 */

/*
input: Bless, Hukporti
Hashes_Result: 570412ec3f38069ae3caf4c266383ecd6cb726b82a5beee5b5491ba52baee465 */
