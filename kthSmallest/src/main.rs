fn kth_smallest(nums: Vec<i32>, k: usize) -> i32 {
    let mut nums = nums;
    nums.sort();
    nums[k - 1]
}

// Func execution
fn main() {
    let nums = vec![4, 1, 3, 2, 5];
    let k = 2;
    println!("{}th smallest element: {}", k, kth_smallest(nums, k));
}
