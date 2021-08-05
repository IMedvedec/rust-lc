/// Function used to check if a 32 bit integer is a palindrome.
pub fn is_palindrome(x: i32) -> bool {
    x == reverse_number(x)
}

/// Function used to reverse a 32 bit integer number.
fn reverse_number(mut x: i32) -> i32 {
    let mut stack = Vec::new();

    while x > 0 {
        stack.push(x % 10);
        x /= 10;
    }

    let mut result = 0;
    let mut multiplier = 1;

    while let Some(y) = stack.pop() {
        result += y * multiplier;
        multiplier *= 10;
    }

    result
}
