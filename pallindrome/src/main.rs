fn is_palindrome(s: &str) -> bool {
    s.chars().eq(s.chars().rev())
}

// Test
fn main() {
    let s = "ratrace";
    println!("Is '{}' a palindrome? {}", s, is_palindrome(s));
}
