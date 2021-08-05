use std::collections::HashMap;

pub fn two_sum(nums: &[i32], target: i32) -> (usize, usize) {
    let mut nums_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    for (i, item) in nums.iter().enumerate() {
        if let Some(j) = nums_map.get(&(target - *item)) {
            return (*j, i);
        }

        nums_map.insert(*item, i);
    }

    panic!("No numbers sum to the target!");
}

pub fn two_sum_lc(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::convert::TryInto;

    let mut nums_map: HashMap<i32, usize> = HashMap::with_capacity(nums.len());

    for (i, item) in nums.iter().enumerate() {
        if let Some(j) = nums_map.get(&(target - *item)) {
            return vec![(*j).try_into().unwrap(), i.try_into().unwrap()];
        }

        nums_map.insert(*item, i);
    }

    panic!("No numbers sum to the target!");
}
