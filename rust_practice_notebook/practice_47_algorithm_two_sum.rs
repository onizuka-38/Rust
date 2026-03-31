// Rust practice notebook
// File: practice_47_algorithm_two_sum.rs

fn main() {
    println!("practice 47: algorithm_two_sum");
    demo();
}

fn demo() {
    // TODO: refine this practice example
    let nums = [2,7,11,15]; println!("{:?}", two_sum(&nums, 9));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        demo();
    }
}
fn two_sum(nums: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..nums.len() { for j in (i + 1)..nums.len() { if nums[i] + nums[j] == target { return Some((i, j)); } } }
    None
}

