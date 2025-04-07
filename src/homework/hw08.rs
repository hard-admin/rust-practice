fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false; // Числа <= 1 не є простими
    }
    if n == 2 {
        return true; // 2 — просте число
    }
    if n % 2 == 0 {
        return false; // Парні числа > 2 не є простими
    }

    // Перевіряємо непарні дільники до кореня з n
    for i in (3..=((n as f64).sqrt() as u32)).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let number = 17; // Приклад числа для перевірки
    if is_prime(number) {
        println!("{} є простим числом", number);
    } else {
        println!("{} не є простим числом", number);
    }
}