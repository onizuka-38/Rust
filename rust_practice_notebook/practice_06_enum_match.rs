// Rust practice notebook
// File: practice_06_enum_match.rs

fn main() {
    println!("practice 06: enum_match");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let code = Status::Ok; match code { Status::Ok => println!("ok"), _ => println!("other") }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
enum Status { Ok, Err }

