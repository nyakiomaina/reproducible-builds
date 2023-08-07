extern crate hex;

fn main() {
    let bytes: &[u8] = b"Hello, Rust!";

    let encoded = hex::encode(bytes);
    println!("Encoded: {}", encoded);

    let decoded = hex::decode(&encoded).expect("Decoding failed");
    println!("Decoded: {}", String::from_utf8_lossy(&decoded));
}

