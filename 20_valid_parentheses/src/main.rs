fn main() {
    let input = String::from("[(()())]");
    let result = is_valid(input);
    println!("{:?}", result);
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = vec![];

    for c in s.chars() {
        if c == '(' || c == '[' || c == '{' {
            stack.push(c);
            continue;
        }
        if stack.len() < 1 {
            return false;
        }
        let last = stack.pop().unwrap();
        match c {
            '}' => {
                if last != '{' {
                    return false;
                }
            }
            ']' => {
                if last != '[' {
                    return false;
                }
            }
            ')' => {
                if last != '(' {
                    return false;
                }
            }
            _ => panic!("What the heck is that?"),
        }
    }
    stack.is_empty()
}
