// Rust practice notebook
// File: practice_27_thread_spawn.rs

fn main() {
    println!("practice 27: thread_spawn");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let h = std::thread::spawn(|| 2 + 2); println!("{:?}", h.join());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
