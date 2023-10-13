fn main() {
    let v1 = vec![7, 1, 5, 3, 6, 4];
    let v2 = vec![7, 6, 4, 3, 1];
    println!("{}", max_profit(v1));
    println!("{}", max_profit(v2));
}

fn max_profit(prices: Vec<i32>) -> i32 {
    let mut l: usize = 0;
    let mut r: usize = 1;
    let mut max_diff = 0;
    while r < prices.len() {
        let diff = prices[r] - prices[l];
        if diff > max_diff {
            max_diff = diff;
        }
        if prices[r] < prices[l] {
            l = r;
            r += 1;
        } else {
            r += 1;
        }
    }
    max_diff
}
