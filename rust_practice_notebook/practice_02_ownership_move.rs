// Rust practice notebook
// File: practice_02_ownership_move.rs

fn main() {
    println!("practice 02: ownership_move");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let s = String::from("hello"); let t = s.clone(); println!("{t}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
