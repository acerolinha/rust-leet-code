use std::collections::HashSet;

fn main() {
    let t1 = longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
    let t2 = longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
    println!("{t1}");
    println!("{t2}");
}

fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<i32> = nums.into_iter().collect();
    let mut max_seq = 0;
    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut current = num;
            let mut seq = 1;
            while num_set.contains(&(current + 1)) {
                seq += 1;
                current += 1;
            }
            max_seq = max_seq.max(seq);
        }
    }
    max_seq
}
