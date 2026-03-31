// Rust practice notebook
// File: practice_15_generic_functions.rs

fn main() {
    println!("practice 15: generic_functions");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    println!("max={} ", if 3>8 {3} else {8});
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
