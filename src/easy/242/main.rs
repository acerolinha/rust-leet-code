fn main() {
    let t1 = is_anagram(String::from("anagram"), String::from("nagaram"));
    let t2 = is_anagram(String::from("rat"), String::from("car"));
    println!("{t1}");
    println!("{t2}");
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut char_counts = [0; 26];
    for (s_char, t_char) in s.chars().zip(t.chars()) {
        char_counts[s_char as usize - 'a' as usize] += 1;
        char_counts[t_char as usize - 'a' as usize] -= 1;
    }
    char_counts.iter().all(|&x| x == 0)
}
