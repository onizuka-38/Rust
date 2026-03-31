// Rust practice notebook
// File: practice_40_btreemap_set.rs

fn main() {
    println!("practice 40: btreemap_set");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    use std::collections::{BTreeMap,BTreeSet}; let mut m=BTreeMap::new(); m.insert(1,"a"); let mut s=BTreeSet::new(); s.insert(2); println!("{} {}", m.len(), s.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
