fn main() {
    let mut t1 = vec![1, 2, 3, 0, 0, 0];
    let mut t2 = vec![1];
    let mut t3 = vec![0];
    merge(&mut t1, 3, &mut vec![2, 5, 6], 3);
    merge(&mut t2, 1, &mut vec![], 0);
    merge(&mut t3, 0, &mut vec![1], 1);
    println!("{t1:?}");
    println!("{t2:?}");
    println!("{t3:?}");
}

fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    for i in (m as usize)..(n + m) as usize {
        nums1[i as usize] = nums2[i - m as usize];
    }
    nums1.sort_unstable();
}
