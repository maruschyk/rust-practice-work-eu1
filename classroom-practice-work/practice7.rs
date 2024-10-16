fn main() {
    let triangles = 5; // Кількість трикутників, ви можете змінити це значення

    draw_tree(triangles);
}

fn draw_tree(triangles: usize) {
    let height = triangles + 1; // Висота ялинки

    (0..height).for_each(|level| {
        let width = (level * 2) + 1; // Ширина трикутника
        let spaces = " ".repeat((height * 2 - 1 - width) / 2); // Пробіли зліва

        // Малюємо трикутник
        let triangle = (0..width).map(|_| '*').collect::<String>();
        println!("{}{}", spaces, triangle);
    });

    // Малюємо стовбур
    let trunk_width = 3; // Ширина стовбура
    let trunk_spaces = " ".repeat((height * 2 - 1 - trunk_width) / 2);
    println!("{}{}", trunk_spaces, "|||");
}
