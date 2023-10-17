fn main() {
    let t1 = String::from("A man, a plan, a canal: Panama");
    let t2 = String::from("race a car");
    let t3 = String::from(" ");
    println!("{}", is_palindrome(t1));
    println!("{}", is_palindrome(t2));
    println!("{}", is_palindrome(t3));
}

fn is_palindrome(s: String) -> bool {
    let s: String = s
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();
    let v: String = s.chars().rev().collect();
    s.eq(&v)
}
