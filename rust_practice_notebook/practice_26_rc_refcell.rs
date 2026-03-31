// Rust practice notebook
// File: practice_26_rc_refcell.rs

fn main() {
    println!("practice 26: rc_refcell");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    use std::cell::RefCell; use std::rc::Rc; let v = Rc::new(RefCell::new(1)); *v.borrow_mut() += 1; println!("{}", v.borrow());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
