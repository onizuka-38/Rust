// Rust practice notebook
// File: practice_16_trait_basics.rs

fn main() {
    println!("practice 16: trait_basics");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    println!("trait basics practice");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
