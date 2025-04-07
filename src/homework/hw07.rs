fn swap_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else if c.is_lowercase() {
                c.to_uppercase().next().unwrap()
            } else {
                c // Залишаємо не-літери без змін
            }
        })
        .collect()
}

fn main() {
    let text = "Hello, World! 123";
    let result = swap_case(text);
    println!("Original: {}", text);
    println!("Swapped:  {}", result);
}