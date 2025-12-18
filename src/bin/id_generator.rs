// src/bin/id_generator.rs
use uuid::Uuid;

fn main() {
    // In your seed script or bin/id_generator
    let id = Uuid::now_v7();
    println!("UUID (for BLOB): \nX'{}'", hex::encode(id.as_bytes()));
}
