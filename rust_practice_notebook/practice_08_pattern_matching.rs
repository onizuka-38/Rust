// Rust practice notebook
// File: practice_08_pattern_matching.rs

fn main() {
    println!("practice 08: pattern_matching");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let pair = (2, 7); let (a, b) = pair; println!("{}", a + b);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
