fn first_occurrence(nums: &[i32], target: i32) -> Option<usize> {
    nums.iter().position(|&x| x == target)
}

// Function execution
fn main() {
    let nums = vec![1, 2, 3, 4, 4, 5];
    let target = 4;
    println!("First occurrence of {} is at index {:?}", target, first_occurrence(&nums, target));
}
