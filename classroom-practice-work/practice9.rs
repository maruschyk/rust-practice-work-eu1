fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false; // числа менше 2 не є простими
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false; // якщо число ділиться на i, воно не є простим
        }
    }
    true // якщо жоден дільник не знайдено, число просте
}

fn main() {
    let num = 29; // приклад числа для перевірки
    if is_prime(num) {
        println!("{} є простим числом.", num);
    } else {
        println!("{} не є простим числом.", num);
    }
}
