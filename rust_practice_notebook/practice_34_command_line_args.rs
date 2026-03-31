// Rust practice notebook
// File: practice_34_command_line_args.rs

fn main() {
    println!("practice 34: command_line_args");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let args: Vec<String> = std::env::args().collect(); println!("argc={}", args.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
