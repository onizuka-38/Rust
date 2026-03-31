// Rust practice notebook
// File: practice_01_variables_and_mutability.rs

fn main() {
    println!("practice 01: variables_and_mutability");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let mut x = 10; x += 5; println!("x={x}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
