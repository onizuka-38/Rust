// Rust practice notebook
// File: practice_07_option_result.rs

fn main() {
    println!("practice 07: option_result");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let v: Option<i32> = Some(3); println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
