// Rust practice notebook
// File: practice_14_lifetimes_intro.rs

fn main() {
    println!("practice 14: lifetimes_intro");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let msg = String::from("life"); println!("{}", msg.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
