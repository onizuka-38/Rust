// Rust practice notebook
// File: practice_22_path_and_osstr.rs

fn main() {
    println!("practice 22: path_and_osstr");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let p = std::path::Path::new("./a/b"); println!("{}", p.display());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
