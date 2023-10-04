use std::collections::HashSet;

fn main() {
    println!("{}", contains_duplicate(vec![1, 2, 3, 1]));
    println!("{}", contains_duplicate(vec![1, 2, 3, 4]));
    println!("{}", contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));
}

fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut num_set = HashSet::with_capacity(nums.len());
    for num in nums {
        if !num_set.insert(num) {
            return true;
        }
    }
    false
}
