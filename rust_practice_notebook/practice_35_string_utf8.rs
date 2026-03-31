// Rust practice notebook
// File: practice_35_string_utf8.rs

fn main() {
    println!("practice 35: string_utf8");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let s = "hello"; println!("bytes={}", s.as_bytes().len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
