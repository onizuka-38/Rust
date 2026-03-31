// Rust practice notebook
// File: practice_28_mpsc_channel.rs

fn main() {
    println!("practice 28: mpsc_channel");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    use std::sync::mpsc; let (tx, rx)=mpsc::channel(); tx.send(7).unwrap(); println!("{}", rx.recv().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
