// Rust practice notebook
// File: practice_21_bufread_lines.rs

fn main() {
    println!("practice 21: bufread_lines");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let lines = vec!["a","b","c"]; for l in lines { println!("{l}"); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
