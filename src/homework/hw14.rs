fn gray(n: u8) -> Vec<String> {
    if n == 0 {
        return vec!["".to_string()];
    }

    // Генеруємо всі комбінації для n бітів
    let count = 1 << n; // 2^n
    let mut result = Vec::with_capacity(count);

    for i in 0..count {
        // Для класичного сірого коду: i ^ (i >> 1)
        let gray_code = i ^ (i >> 1);
        // Перетворюємо в бінарний рядок із n бітів
        let mut binary = format!("{:0width$b}", gray_code, width = n as usize);
        result.push(binary);
    }

    result
}

fn main() {
    let test_data = vec![
        (0, vec![""]),
        (1, vec!["0", "1"]),
        (2, vec!["00", "01", "11", "10"]),
        (3, vec!["000", "001", "011", "010", "110", "111", "101", "100"]),
        (4, vec![
            "0000", "0001", "0011", "0010", "0110", "0111", "0101", "0100",
            "1100", "1101", "1111", "1110", "1010", "1011", "1001", "1000",
        ]),
    ];

    for (n, expected) in test_data.iter() {
        let result = gray(*n);
        println!("gray({}) = {:?}", n, result);
        assert_eq!(result, *expected);
    }
}