// Rust practice notebook
// File: practice_09_vec_practice.rs

fn main() {
    println!("practice 09: vec_practice");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let mut v = vec![1,2,3]; v.push(4); println!("{:?}", v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
