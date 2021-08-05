mod two_sum;

fn main() {
    let nums = [3, 3];
    let target = 6;

    let (i, j) = two_sum::two_sum(&nums, target);

    println!(
        "Numbers on indices that sum on target are: {} and {}.",
        i, j
    )
}
