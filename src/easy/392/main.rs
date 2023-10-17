fn main() {
    let t1 = is_subsequence(String::from("abc"), String::from("ahbgdc"));
    let t2 = is_subsequence(String::from("axc"), String::from("ahbgdc"));
    let t3 = is_subsequence(String::from(""), String::from("ahbgdc"));
    println!("{}", t1);
    println!("{}", t2);
    println!("{}", t3);
}
fn is_subsequence(s: String, t: String) -> bool {
    let mut l = 0;
    let mut r = 0;
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();
    while l < s.len() && r < t.len() {
        if s[l] == t[r] {
            l += 1;
        }
        r += 1;
    }
    l == s.len()
}
