// Rust practice notebook
// File: practice_24_use_and_crate.rs

fn main() {
    println!("practice 24: use_and_crate");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    println!("use crate::x style practice");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
