fn merge_sorted_arrays(nums1: &mut Vec<i32>, nums2: Vec<i32>) {
    let mut i = nums1.len() as i32 - 1;
    let mut j = nums2.len() as i32 - 1;
    let mut k = nums1.len() as i32 + nums2.len() as i32 - 1;

    while j >= 0 {
        if i >= 0 && nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }
}

// Test
fn main() {
    let mut nums1 = vec![1, 3, 5, 0, 0, 0];
    let nums2 = vec![2, 4, 6];
    merge_sorted_arrays(&mut nums1, nums2);
    println!("Merged array: {:?}", nums1);
}
