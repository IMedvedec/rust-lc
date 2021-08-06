/// Returns the 32 bit integer as reversed.
/// If the reversed integer would overflow, return 0.
pub fn reverse(x: i32) -> i32 {
    let mut mut_x = x;

    let mut stack = Vec::new();
    while mut_x != 0 {
        stack.push(mut_x % 10);
        mut_x /= 10;
    }

    let mut result = 0_i32;
    let mut multiplier = 1_i32;

    while let Some(y) = stack.pop() {
        result = match y.checked_mul(multiplier) {
            None => return 0,
            Some(yy) => match result.checked_add(yy) {
                None => return 0,
                Some(yyy) => yyy,
            },
        };

        multiplier *= 10;
    }

    result
}

/// Represents the same functionality as reverse.
/// Function is written in funcitonal paradigm which gives a classy solution.
pub fn reverse_functional(x: i32) -> i32 {
    x.abs()
        .to_string()
        .chars()
        .rev()
        .collect::<String>()
        .parse::<i32>()
        .map(|r| r * x.signum())
        .unwrap_or(0)
}
