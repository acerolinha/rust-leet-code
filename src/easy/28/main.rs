fn main() {
    let t1 = str_str(String::from("sadbutsad"), String::from("sad"));
    let t2 = str_str(String::from("leetcode"), String::from("leeto"));

    println!("{}", t1);
    println!("{}", t2);
}

fn str_str(haystack: String, needle: String) -> i32 {
    match haystack.find(&needle) {
        Some(i) => i as i32,
        _ => -1,
    }
}
