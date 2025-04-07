// Функція для перевірки, чи всі цифри різні
fn all_different(digits: &[u8]) -> bool {
    let mut used = [false; 10];
    for &d in digits {
        if used[d as usize] {
            return false;
        }
        used[d as usize] = true;
    }
    true
}

// Функція для знаходження рішень і виведення результату
fn solve_cryptarithm() -> Vec<(u8, u8, u8, u8, u8, u8, u8, u8)> {
    let mut solutions = Vec::new();

    // Перебираємо всі можливі значення для m, u, x, a, s, l, o, n
    for m in 1..=9 { // m не може бути 0
        for u in 0..=9 {
            for x in 0..=9 {
                for a in 0..=9 {
                    for s in 1..=9 { // s не може бути 0
                        for l in 0..=9 {
                            for o in 0..=9 {
                                for n in 0..=9 {
                                    // Перевіряємо, чи всі цифри різні
                                    let digits = [m, u, x, a, s, l, o, n];
                                    if !all_different(&digits) {
                                        continue;
                                    }

                                    // Обчислюємо значення чисел
                                    let muxa = m as i32 * 1000 + u as i32 * 100 + x as i32 * 10 + a as i32;
                                    let x_val = x as i32;
                                    let a_val = a as i32;
                                    let slon = s as i32 * 1000 + l as i32 * 100 + o as i32 * 10 + n as i32;

                                    // Перевіряємо рівняння
                                    if muxa - x_val * a_val == slon {
                                        solutions.push((m, u, x, a, s, l, o, n));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Виводимо всі рішення у заданому форматі
    for (i, &(m, u, x, a, s, l, o, n)) in solutions.iter().enumerate() {
        println!("Solution {}:", i + 1);
        println!("{}{}{}{}", m, u, x, a); // muxa
        println!("{}    {}", x, a);       // x    a
        println!("------");
        println!("  {}{}{}{}", s, l, o, n); // slon
        println!();
    }

    solutions
}

fn main() {
    let solutions = solve_cryptarithm();
    println!("Total number of solutions: {}", solutions.len());
}