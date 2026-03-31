// Rust practice notebook
// File: practice_10_hashmap_practice.rs

fn main() {
    println!("practice 10: hashmap_practice");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    use std::collections::HashMap; let mut m = HashMap::new(); m.insert("k", 1); println!("{:?}", m.get("k"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
