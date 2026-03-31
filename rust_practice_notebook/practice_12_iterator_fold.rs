// Rust practice notebook
// File: practice_12_iterator_fold.rs

fn main() {
    println!("practice 12: iterator_fold");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let sum = [1,2,3,4].iter().fold(0, |acc, x| acc + x); println!("{sum}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
