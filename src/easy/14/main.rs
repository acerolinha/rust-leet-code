fn main() {
    let s1 = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let s2 = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
    let s3 = vec!["a".to_string()];
    let s4 = vec!["ab".to_string(), "a".to_string()];
    let s5 = vec!["cir".to_string(), "car".to_string()];
    println!("{}", longest_common_prefix(s1));
    println!("{}", longest_common_prefix(s2));
    println!("{}", longest_common_prefix(s3));
    println!("{}", longest_common_prefix(s4));
    println!("{}", longest_common_prefix(s5));
}

fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.into_iter()
        .reduce(|acc, cur| {
            acc.chars()
                .zip(cur.chars())
                .take_while(|(a, c)| a == c)
                .map(|(c, _)| c)
                .collect()
        })
        .unwrap()
}
