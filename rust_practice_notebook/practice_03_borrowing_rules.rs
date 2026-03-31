// Rust practice notebook
// File: practice_03_borrowing_rules.rs

fn main() {
    println!("practice 03: borrowing_rules");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let s = String::from("borrow"); println!("len={}", s.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
