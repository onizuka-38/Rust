// Rust practice notebook
// File: practice_25_smart_pointer_box.rs

fn main() {
    println!("practice 25: smart_pointer_box");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let n = Box::new(10); println!("{n}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
