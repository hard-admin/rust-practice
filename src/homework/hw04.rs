const W: usize = 9; // Ширина ромба (максимальна, непарне число, 10..80)
const H: usize = 5; // Висота ромба (непарне число, 10..80)

fn main() {
    let mut output = String::new();

    // Перевірка, що H і W непарні та в межах
    assert!(W >= 10 && W <= 80 && W % 2 == 1, "W має бути непарним у діапазоні 10..80");
    assert!(H >= 10 && H <= 80 && H % 2 == 1, "H має бути непарним у діапазоні 10..80");

    let mid = H / 2; // Середина ромба

    for i in 0..H {
        let spaces_before = if i <= mid { mid - i } else { i - mid };
        let stars = if i <= mid { 2 * i + 1 } else { 2 * (H - i - 1) + 1 };
        let spaces_inside = if stars > 1 { W - 2 - 2 * spaces_before } else { 0 };

        // Додаємо пробіли перед зірочками
        for _ in 0..spaces_before {
            output.push(' ');
        }
        // Перша зірочка
        output.push('*');
        // Пробіли всередині (якщо є)
        if stars > 1 {
            for _ in 0..spaces_inside {
                output.push(' ');
            }
            output.push('*');
        }
        // Перехід на новий рядок
        output.push('\n');
    }

    print!("{}", output);
    println!(); // Порожній рядок у кінці
}