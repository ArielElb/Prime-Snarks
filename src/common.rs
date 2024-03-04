use blake2::{Blake2s, Digest};

fn hash_number(number: u64) -> [u8; 32] {
    // Create a Blake2s hasher
    let mut hasher = Blake2s::new();

    // Update the hasher with the bytes of the input number
    hasher.update(&number.to_le_bytes());

    // Finalize the hash and return the result as a fixed-size array
    hasher.finalize().into()
}


