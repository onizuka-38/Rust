// Rust practice notebook
// File: practice_44_integration_like_test.rs

fn main() {
    println!("practice 44: integration_like_test");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    assert!("rust".contains("ru"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
