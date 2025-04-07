const W: usize = 20; // Ширина конверта (10..80)
const H: usize = 10; // Висота конверта (10..80)

fn main() {
    let mut output = String::new();

    for i in 0..H {
        for j in 0..W {
            if i == 0 || i == H - 1 {
                // Верхня і нижня межі
                output.push(if j == 0 || j == W - 1 { '+' } else { '-' });
            } else if j == 0 || j == W - 1 {
                // Бокові межі
                output.push('|');
            } else if j == i * (W - 2) / (H - 2) + 1 {
                // Ліва діагональ (\)
                output.push('\\');
            } else if j == (W - 2) - (i - 1) * (W - 2) / (H - 2) {
                // Права діагональ (/)
                output.push('/');
            } else {
                // Порожній простір
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
    println!(); // Додаємо порожній рядок у кінці
}