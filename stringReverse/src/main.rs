fn reverse_string(s: &mut Vec<char>) {
    let mut left = 0;
    let mut right = s.len() - 1;
    while left < right {
        s.swap(left, right);
        left += 1;
        right -= 1;
    }
}

// Func execn
fn main() {
    let mut s = vec!['h', 'e', 'l', 'l', 'o'];
    reverse_string(&mut s);
    println!("Reversed string: {:?}", s);
}
