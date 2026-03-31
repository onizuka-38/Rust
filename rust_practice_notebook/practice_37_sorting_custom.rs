// Rust practice notebook
// File: practice_37_sorting_custom.rs

fn main() {
    println!("practice 37: sorting_custom");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let mut v = vec![3,1,2]; v.sort(); println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
