// Rust practice notebook
// File: practice_50_builder_pattern.rs

fn main() {
    println!("practice 50: builder_pattern");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let user = UserBuilder::new().id(1).name("trinity").build(); println!("{} {}", user.id, user.name);
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
#[derive(Debug)]
struct UserBuilder { id: i32, name: String }
impl UserBuilder { fn new() -> Self { Self { id: 0, name: String::new() } } fn id(mut self, id: i32) -> Self { self.id = id; self } fn name(mut self, name: &str) -> Self { self.name = name.to_string(); self } fn build(self) -> User { User { id: self.id, name: self.name } } }

