use rand::Rng;

// Генерує рандомний вектор довжиною n зі значеннями в діапазоні [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Знаходить мінімальну суму сусідніх елементів і їхні індекси
fn min_adjacent_sum(data: &[i32]) -> (i32, (usize, usize)) {
    if data.len() < 2 {
        return (0, (0, 0)); // Якщо вектор замалий, повертаємо 0 і (0,0)
    }

    let mut min_sum = data[0] + data[1];
    let mut min_indices = (0, 1);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indices = (i, i + 1);
        }
    }

    (min_sum, min_indices)
}

// Виводить вектор і результат у заданому форматі
fn print_result(data: &[i32], min_sum: i32, indices: (usize, usize)) {
    // Виводимо індекси
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:2} ", i);
    }
    println!();

    // Виводимо дані
    print!("data:   [");
    for i in 0..data.len() {
        if i == data.len() - 1 {
            print!("{}", data[i]);
        } else {
            print!("{}, ", data[i]);
        }
    }
    println!("]");

    // Виводимо підкреслення
    print!("indexes: ");
    for i in 0..data.len() {
        if i == indices.0 {
            print!("_______");
            break;
        } else {
            print!("   ");
        }
    }
    println!();

    // Виводимо результат
    println!(
        "min adjacent sum={} at indexes:{:?}",
        min_sum, indices
    );
}

fn main() {
    // Генеруємо вектор довжиною 20
    let data = gen_random_vector(20);
    
    // Знаходимо мінімальну суму сусідніх елементів
    let (min_sum, indices) = min_adjacent_sum(&data);
    
    // Виводимо результат
    print_result(&data, min_sum, indices);
}