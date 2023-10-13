fn main() {
    let mut v1 = vec![1, 1, 2];
    let mut v2 = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    println!("{:?} | {:?}", remove_duplicates(&mut v1), v1);
    println!("{:?} | {:?}", remove_duplicates(&mut v2), v2);
}

fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}
