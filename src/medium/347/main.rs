use std::collections::HashMap;

fn main() {
    let t1 = top_k_frequent(vec![1, 1, 1, 2, 2, 3], 2);
    let t2 = top_k_frequent(vec![1], 1);
    println!("{:?}", t1);
    println!("{:?}", t2);
}

fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut freq_map: HashMap<i32, usize> = HashMap::new();
    let mut max_freq: usize = 0;
    for num in nums {
        let freq = freq_map.entry(num).or_insert(0);
        *freq += 1;
        max_freq = max_freq.max(*freq as usize);
    }
    let mut most_frequent: Vec<Vec<i32>> = vec![vec![]; max_freq];
    for (num, freq) in freq_map {
        most_frequent[freq - 1].push(num);
    }
    most_frequent
        .into_iter()
        .rev()
        .flat_map(|x| x.into_iter())
        .take(k as usize)
        .collect()
}
