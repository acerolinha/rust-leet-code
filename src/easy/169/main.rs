fn main() {
    let v1 = vec![3, 2, 3];
    let v2 = vec![2, 2, 1, 1, 1, 2, 2];
    println!("{:?}", majority_element(v1));
    println!("{:?}", majority_element(v2));
}

fn majority_element(nums: Vec<i32>) -> i32 {
    let mut m = nums[0];
    let mut count = 1;
    for i in 1..nums.len() {
        if count == 0 {
            m = nums[i];
            count += 1;
        } else if m == nums[i] {
            count += 1;
        } else {
            count -= 1;
        }
    }
    m
}
