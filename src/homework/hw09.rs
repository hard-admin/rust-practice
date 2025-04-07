fn rotate(s: String, n: isize) -> String {
    if s.is_empty() || n == 0 {
        return s;
    }

    let len = s.len() as isize;
    // Обчислюємо ефективний зсув, враховуючи від'ємні значення
    let shift = ((n % len) + len) % len; // Нормалізуємо зсув
    let shift = shift as usize;

    // Розділяємо рядок на дві частини і міняємо їх місцями
    let (end, start) = s.split_at(len as usize - shift);
    format!("{}{}", start, end)
}

fn main() {
    let s = "abcdefg".to_string();
    let shifts = [
        (0, "abcdefg"),
        (1, "habcdef"),
        (2, "ghabcde"),
        (10, "habcdef"),
        (-1, "bcdefga"),
        (-2, "cdefgab"),
        (-10, "cdefgab"),
    ];

    for (n, expected) in shifts.iter() {
        let result = rotate(s.clone(), *n);
        println!("rotate(\"{}\", {}) = \"{}\", expected: \"{}\"", s, n, result, expected);
        assert_eq!(result, *expected);
    }
}