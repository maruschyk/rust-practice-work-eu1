use std::collections::HashSet;

fn main() {
    let mut count = 0;

    for m in 1..=8 {
        for u in 1..=8 {
            for x in 1..=8 {
                for a in 1..=8 {
                    for s in 1..=8 {
                        for l in 1..=8 {
                            for o in 1..=8 {
                                for n in 1..=8 {
                                    // Перевіряємо, чи всі числа унікальні
                                    let numbers = [m, u, x, a, s, l, o, n];
                                    if numbers.iter().copied().collect::<HashSet<_>>().len() == 8 {
                                        let muxa = m * 1000 + u * 100 + x * 10 + a;
                                        let slon = s * 1000 + l * 100 + o * 10 + n;
                                        
                                        if muxa * x == slon {
                                            count += 1;
                                            println!("Solution {}: muxa = {}, slon = {}", count, muxa, slon);
                                            println!("  {}{}{}{}", m, u, x, a); // Виправлено: чотири аргументи
                                            println!("x        {}", a);
                                            println!("  ------");
                                            println!("    {}{}{}{}", s, l, o, n);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Total solutions: {}", count);
}
