// Rust practice notebook
// File: practice_36_byte_and_bit_ops.rs

fn main() {
    println!("practice 36: byte_and_bit_ops");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let x:u8 = 0b1010; println!("{:b}", x << 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
