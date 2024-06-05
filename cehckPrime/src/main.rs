fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false;
    }
    for i in 2..=num / 2 {
        if num % i == 0 {
            return false;
        }
    }
    true
}

// Func execution
fn main() {
    let num = 17;
    println!("Is {} prime? {}", num, is_prime(num));
}
