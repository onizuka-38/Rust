// Rust practice notebook
// File: practice_33_serde_json_parse.rs

fn main() {
    println!("practice 33: serde_json_parse");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let raw = r#"{""a"":1}"#; println!("{raw}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
