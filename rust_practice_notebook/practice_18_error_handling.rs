// Rust practice notebook
// File: practice_18_error_handling.rs

fn main() {
    println!("practice 18: error_handling");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let r: Result<i32,&str> = Ok(42); println!("{:?}", r);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
