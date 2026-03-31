// Rust practice notebook
// File: practice_29_mutex_arc.rs

fn main() {
    println!("practice 29: mutex_arc");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    use std::sync::{Arc,Mutex}; let c = Arc::new(Mutex::new(0)); *c.lock().unwrap() += 1; println!("{}", c.lock().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
