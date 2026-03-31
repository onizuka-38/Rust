// Rust practice notebook
// File: practice_49_mini_parser.rs

fn main() {
    println!("practice 49: mini_parser");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let expr = "1+2+3"; println!("sum={}", parse_plus(expr));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
fn parse_plus(expr: &str) -> i32 { expr.split('+').filter_map(|x| x.trim().parse::<i32>().ok()).sum() }

