// Rust practice notebook
// File: practice_17_trait_impl.rs

fn main() {
    println!("practice 17: trait_impl");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    println!("trait impl practice");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
