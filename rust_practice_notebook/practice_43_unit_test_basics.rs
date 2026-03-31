// Rust practice notebook
// File: practice_43_unit_test_basics.rs

fn main() {
    println!("practice 43: unit_test_basics");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    assert_eq!(2+2,4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
