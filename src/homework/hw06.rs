const TRIANGLES: usize = 3; // Кількість трикутників

fn main() {
    let mut output = String::new();
    
    // Висота одного трикутника (фіксована, наприклад, 3 рядки)
    let triangle_height = 3;
    let total_height = TRIANGLES * triangle_height;

    // Малюємо ялинку за допомогою ітератора
    for i in 0..total_height {
        let triangle_idx = i / triangle_height; // Номер поточного трикутника
        let row_in_triangle = i % triangle_height; // Рядок у поточному трикутнику
        
        // Кількість пробілів перед зірочками
        let spaces = total_height - triangle_idx - row_in_triangle - 1;
        output.extend(std::iter::repeat(' ').take(spaces));
        
        // Кількість зірочок у рядку
        let stars = 2 * row_in_triangle + 1 + 2 * triangle_idx;
        output.extend(std::iter::repeat('*').take(stars));
        
        // Перехід на новий рядок
        output.push('\n');
    }

    // Додаємо стовбур
    let trunk_spaces = total_height - 1;
    output.extend(std::iter::repeat(' ').take(trunk_spaces));
    output.push_str("*\n");

    println!("{}", output);
}