fn is_palindrome(x: u32) -> bool {
    // Перетворюємо число в рядок для зручності
    let s = x.to_string();
    // Порівнюємо символи з початку і кінця
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    let data = [
        (123, false),
        (121, true),
        (1221, true),
    ];

    for (n, expected) in data.iter() {
        let result = is_palindrome(*n);
        println!("is_palindrome({}) = {}, expected: {}", n, result, expected);
        assert_eq!(result, *expected);
    }
}