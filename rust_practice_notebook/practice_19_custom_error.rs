// Rust practice notebook
// File: practice_19_custom_error.rs

fn main() {
    println!("practice 19: custom_error");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let _parsed = "123".parse::<u32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
