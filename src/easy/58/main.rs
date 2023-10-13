fn main() {
    let s1 = String::from("Hello World");
    let s2 = String::from("   fly me   to   the moon  ");
    let s3 = String::from("luffy is still joyboy");
    println!("{}", length_of_last_word(s1));
    println!("{}", length_of_last_word(s2));
    println!("{}", length_of_last_word(s3));
}

fn length_of_last_word(s: String) -> i32 {
    s.trim_end()
        .chars()
        .rev()
        .take_while(|c| c.is_alphabetic())
        .count() as i32
}
