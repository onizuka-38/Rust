// Rust practice notebook
// File: practice_13_closures_basics.rs

fn main() {
    println!("practice 13: closures_basics");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let add = |a:i32,b:i32| a+b; println!("{}", add(2,3));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
