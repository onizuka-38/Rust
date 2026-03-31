// Rust practice notebook
// File: practice_39_linked_list_std.rs

fn main() {
    println!("practice 39: linked_list_std");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    use std::collections::LinkedList; let mut l=LinkedList::new(); l.push_back(1); println!("{:?}", l.front());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
