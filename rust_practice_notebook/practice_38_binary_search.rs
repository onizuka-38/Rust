// Rust practice notebook
// File: practice_38_binary_search.rs

fn main() {
    println!("practice 38: binary_search");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let v = [1,3,5,7]; println!("{:?}", v.binary_search(&5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
