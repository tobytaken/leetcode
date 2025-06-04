fn main() {
    let input = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
        String::from("f"),
    ];
    let result = longest_common_prefix(input);
    println!("{:?}", result);
}

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut result: Vec<char> = strs[0].chars().collect();

    for str in strs {
        let str_len = str.len();

        result.truncate(str_len);

        for (i, c) in str.chars().enumerate() {
            if i >= result.len() {
                break;
            }
            if result[i] != c {
                result.truncate(i);
                break;
            }
        }
    }
    result.iter().collect()
}
