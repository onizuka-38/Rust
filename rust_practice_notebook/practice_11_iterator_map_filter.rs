// Rust practice notebook
// File: practice_11_iterator_map_filter.rs

fn main() {
    println!("practice 11: iterator_map_filter");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let nums = [1,2,3,4,5]; let out: Vec<_> = nums.iter().map(|x| x * 2).collect(); println!("{:?}", out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
