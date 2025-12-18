// src/bin/id_generator.rs
use uuid::Uuid;

fn main() {
    let id = Uuid::now_v7();
    println!("{}", id);
}
