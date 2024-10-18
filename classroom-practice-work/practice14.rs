struct Point {
    x: i32,
    y: i32,
}

struct Rectangle {
    a: Point,
    b: Point,
}

// Функція для обчислення площі перекриття між двома прямокутниками
fn overlap_area(rect1: &Rectangle, rect2: &Rectangle) -> i32 {
    let x_overlap = std::cmp::max(0, std::cmp::min(rect1.b.x, rect2.b.x) - std::cmp::max(rect1.a.x, rect2.a.x));
    let y_overlap = std::cmp::max(0, std::cmp::min(rect1.a.y, rect2.a.y) - std::cmp::max(rect1.b.y, rect2.b.y));
    x_overlap * y_overlap
}

// Функція для обчислення фактичної зайнятої площі
fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;
    let mut overlaps = vec![0; xs.len()];

    // Обчислюємо площу кожного прямокутника
    for (i, rect) in xs.iter().enumerate() {
        let width = (rect.b.x - rect.a.x).abs();
        let height = (rect.a.y - rect.b.y).abs();
        total_area += width * height;

        // Обчислюємо площі перекриттів з іншими прямокутниками
        for j in 0..i {
            overlaps[i] += overlap_area(&xs[i], &xs[j]);
        }
    }

    // Віднімемо загальну площу перекриттів
    let total_overlap: i32 = overlaps.iter().sum();
    total_area - total_overlap
}

// Тестові дані
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

// Тестова функція
fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

// Виконання тесту
fn main() {
    area_occupied_test();
    println!("Test passed!");
}
