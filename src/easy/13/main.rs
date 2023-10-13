fn main() {
    let t1 = roman_to_int(String::from("III"));
    let t2 = roman_to_int(String::from("LVIII"));
    let t3 = roman_to_int(String::from("MCMXCIV"));
    println!("{t1}");
    println!("{t2}");
    println!("{t3}");
}

fn roman_to_int(s: String) -> i32 {
    let mut sum = 0;
    let mut iter = s.chars().rev();
    let mut prev_value = ' ';
    while let Some(c) = iter.next() {
        match c {
            'I' => match prev_value {
                'V' | 'X' => sum -= 1,
                _ => sum += 1,
            },
            'V' => sum += 5,
            'X' => match prev_value {
                'L' | 'C' => sum -= 10,
                _ => sum += 10,
            },
            'L' => sum += 50,
            'C' => match prev_value {
                'D' | 'M' => sum -= 100,
                _ => sum += 100,
            },
            'D' => sum += 500,
            'M' => sum += 1000,
            _ => (),
        }
        prev_value = c;
    }
    sum
}
