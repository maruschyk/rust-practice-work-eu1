// Функція для обертання рядка на n позицій
fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    
    if len == 0 {
        return s; // Повертаємо порожній рядок, якщо вхідний рядок порожній
    }
    
    // Зменшуємо n до діапазону [0, len)
    let n = ((n % len as isize) + len as isize) % len as isize;

    let split_point = len - n as usize;
    let (first, second) = s.split_at(split_point);
    
    format!("{}{}", second, first)
}

// Тестові кейси для функції
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(rotate(s.to_string(), *n), exp.to_string());
        });
    }
}

fn main() {
    // Демо використання функції
    let test_string = "abcdefgh".to_string();
    let shifts = 2;
    println!("Rotated string: {}", rotate(test_string, shifts));
}
