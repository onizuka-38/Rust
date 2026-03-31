// Rust practice notebook
// File: practice_23_modules_visibility.rs

fn main() {
    println!("practice 23: modules_visibility");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    println!("module visibility practice");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
