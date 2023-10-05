use std::collections::HashMap;

fn main() {
    let t1 = group_anagrams(vec![
        String::from("eat"),
        String::from("tea"),
        String::from("tan"),
        String::from("ate"),
        String::from("nat"),
        String::from("bat"),
    ]);
    let t2 = group_anagrams(vec![String::from("")]);
    let t3 = group_anagrams(vec![String::from("a")]);
    println!("{:?}", t1);
    println!("{:?}", t2);
    println!("{:?}", t3);
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut word_map: HashMap<Vec<u8>, Vec<String>> = HashMap::with_capacity(strs.len());
    for s in strs {
        let mut key = vec![0; 26];
        for chr in s.chars() {
            key[(chr as u8 - 'a' as u8) as usize] += 1;
        }
        word_map.entry(key).or_insert(Vec::new()).push(s);
    }
    word_map.into_values().collect()
}
