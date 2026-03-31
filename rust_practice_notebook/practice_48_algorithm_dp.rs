// Rust practice notebook
// File: practice_48_algorithm_dp.rs

fn main() {
    println!("practice 48: algorithm_dp");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let n=8usize; println!("fib({n})={}", fib(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
fn fib(n: usize) -> usize { if n < 2 { return n; } let mut a=0usize; let mut b=1usize; for _ in 2..=n { let c=a+b; a=b; b=c; } b }

