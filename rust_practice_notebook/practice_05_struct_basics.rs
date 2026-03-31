// Rust practice notebook
// File: practice_05_struct_basics.rs

fn main() {
    println!("practice 05: struct_basics");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let user = User { id: 1, name: String::from("neo") }; println!("{} {}", user.id, user.name);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
#[derive(Debug)]
struct User { id: i32, name: String }

