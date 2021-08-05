mod palindrome_number;

fn main() {
    let x = -101;

    let result = palindrome_number::is_palindrome(x);

    println!("Is {} palindrome? {}", x, result)
}
