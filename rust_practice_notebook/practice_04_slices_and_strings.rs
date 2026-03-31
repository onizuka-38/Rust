// Rust practice notebook
// File: practice_04_slices_and_strings.rs

fn main() {
    println!("practice 04: slices_and_strings");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let s = String::from("abcdef"); let part = &s[0..3]; println!("{part}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
