use ::std::collections::VecDeque;

fn main() {
    let len = length_of_longest_substring("abcadcdb".to_string());
    println!("{}", len);
}

// Not the most efficient solution - O(n^2)
// Can be done in O(n) by using a fixed-size array for O(1) lookups instead of VecDeque::contains()
// which is an O(n) lookup.
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut seen_chars: VecDeque<char> = VecDeque::new();
    let mut longest_str = 0;

    s.chars().for_each(|c| {
        println!("{:?}", seen_chars);
        if seen_chars.contains(&c) {
            loop {
                if let Some(v) = seen_chars.pop_front() {
                    if v == c {
                        break;
                    }
                }
            }
        }
        seen_chars.push_back(c);
        if seen_chars.len() > longest_str {
            longest_str = seen_chars.len();
        }
    });

    longest_str as i32
}
