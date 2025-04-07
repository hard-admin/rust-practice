// Точка на площині
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

// Прямокутник, заданий двома точками
#[derive(Clone, Copy)]
struct Rectangle {
    a: Point, // Нижня ліва точка
    b: Point, // Верхня права точка
}

impl Rectangle {
    // Обчислює площу прямокутника
    fn area(&self) -> i32 {
        let width = (self.b.x - self.a.x).abs();
        let height = (self.b.y - self.a.y).abs();
        width * height
    }

    // Перевіряє, чи перетинаються два прямокутники, і обчислює площу перетину
    fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        let x1 = self.a.x.max(other.a.x);
        let y1 = self.a.y.max(other.a.y);
        let x2 = self.b.x.min(other.b.x);
        let y2 = self.b.y.min(other.b.y);

        if x1 < x2 && y1 < y2 {
            Some(Rectangle {
                a: Point { x: x1, y: y1 },
                b: Point { x: x2, y: y2 },
            })
        } else {
            None
        }
    }
}

// Обчислює загальну зайняту площу з урахуванням перетинів
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    if xs.is_empty() {
        return 0;
    }

    // Використовуємо принцип включення-виключення для двох прямокутників
    let mut total_area = 0;

    // Додаємо площі всіх прямокутників
    for i in 0..xs.len() {
        total_area += xs[i].area();

        // Віднімаємо площі перетинів для кожної пари
        for j in 0..i {
            if let Some(intersection) = xs[i].intersection(&xs[j]) {
                total_area -= intersection.area();
            }
        }
    }

    total_area
}

// Функція для створення тестових даних
fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn main() {
    let data = test_data();
    let occupied = area_occupied(&data);
    println!("Occupied area: {}", occupied);
    assert_eq!(occupied, 60);
}