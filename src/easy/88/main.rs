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
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = (m + n) - 1;
    while j >= 0 {
        if i >= 0 && nums1[i as usize] > nums2[j as usize] {
            nums1[k as usize] = nums1[i as usize];
            i -= 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
        }
        k -= 1;
    }
}
