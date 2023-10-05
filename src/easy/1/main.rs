use std::collections::HashMap;

fn main() {
    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9));
    println!("{:?}", two_sum(vec![3, 2, 4], 6));
    println!("{:?}", two_sum(vec![3, 3], 6));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_map = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&index) = num_map.get(&complement) {
            return vec![index, i as i32];
        }
        num_map.insert(num, i as i32);
    }
    return vec![0, 0];
}
