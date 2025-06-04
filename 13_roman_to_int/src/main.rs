use std::collections::HashMap;

fn main() {
    let input = String::from("MDXL");
    let result = roman_to_int(input);
    println!("{:?}", result);
}

pub fn roman_to_int(s: String) -> i32 {
    let mapping = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
        ('N', 0),
    ]);
    let mut result = 0;
    let mut prev = 'N';

    for c in s.chars() {
        let curr_val = mapping.get(&c).unwrap();
        let prev_val = mapping.get(&prev).unwrap();

        if prev_val < curr_val {
            result -= prev_val * 2;
        }
        prev = c;
        result += curr_val;
    }
    result
}
