fn main() {
    let mut a1 = vec![3, 2, 2, 3];
    let mut a2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
    println!("{:?} | {:?}", remove_element(&mut a1, 3), a1);
    println!("{:?} | {:?}", remove_element(&mut a2, 2), a2);
}

fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&n| n != val);
    nums.len() as i32
}
