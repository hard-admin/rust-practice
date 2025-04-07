use rand::Rng;

// Рахує мінімальну кількість переміщень вантажу
fn count_permutation(shipments: &Vec<u32>) -> usize {
    if shipments.is_empty() {
        return 0;
    }

    // Обчислюємо суму і середнє
    let total: u32 = shipments.iter().sum();
    let count = shipments.len() as u32;
    if total % count != 0 {
        // Якщо сума не ділиться на кількість кораблів, розподіл неможливий
        panic!("Cannot distribute equally");
    }
    let target = (total / count) as u32;

    // Обчислюємо надлишки/нестачі і підраховуємо переміщення
    let mut moves = 0;
    let mut excess = 0i32; // Накопичений надлишок/нестача

    for &value in shipments {
        let diff = value as i32 - target as i32; // Надлишок (+) або нестача (-)
        excess += diff;
        moves += excess.abs() as usize; // Кількість одиниць, які потрібно перенести
    }

    moves
}

// Генерує вектор вантажів, які можна розподілити однаково
fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let target = rng.gen_range(1..10); // Випадкове середнє значення
    let total = target * n as u32; // Загальна сума, яка ділиться на n

    // Генеруємо вектор із випадковими відхиленнями
    let mut shipments = vec![target; n];
    for i in 0..n {
        let delta = rng.gen_range(-3..4); // Випадкове відхилення
        shipments[i] = (target as i32 + delta).max(0) as u32;
    }

    // Коригуємо, щоб сума була точно total
    let current_sum: u32 = shipments.iter().sum();
    let diff = total as i32 - current_sum as i32;
    shipments[0] = (shipments[0] as i32 + diff).max(0) as u32;

    shipments
}

fn main() {
    // Тестові приклади
    let test1 = vec![8, 2, 2, 4, 4];
    let test2 = vec![9, 3, 7, 2, 9];

    println!("Test 1: {:?}", test1);
    println!("Moves: {}", count_permutation(&test1));

    println!("\nTest 2: {:?}", test2);
    println!("Moves: {}", count_permutation(&test2));

    // Генеруємо випадковий вектор
    let random = gen_shipments(5);
    println!("\nRandom shipments: {:?}", random);
    println!("Moves: {}", count_permutation(&random));
}