// Rust practice notebook
// File: practice_41_const_static.rs

fn main() {
    println!("practice 41: const_static");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    const LIMIT:i32=10; println!("{LIMIT}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
