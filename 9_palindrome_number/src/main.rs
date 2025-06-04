fn main() {
    let result = is_palindrome(11);
    println!("{:?}", result);
}

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut y = x;
    let mut reversed = 0;

    while y > reversed {
        let digit = y % 10;
        reversed = reversed * 10 + digit;
        y /= 10;
    }

    y == reversed || y == reversed / 10
}
