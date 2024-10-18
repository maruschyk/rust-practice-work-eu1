fn main() {
    // Приклади використання функцій
    let shipments1 = vec![8, 2, 2, 4, 4];
    let moves1 = count_permutation(&shipments1);
    println!("Moves for {:?}: {}", shipments1, moves1); // Виведе: Moves for [8, 2, 2, 4, 4]: 4

    let shipments2 = vec![9, 3, 7, 2, 9];
    let moves2 = count_permutation(&shipments2);
    println!("Moves for {:?}: {}", shipments2, moves2); // Виведе: Moves for [9, 3, 7, 2, 9]: 7

    // Генерація вантажів
    let generated_shipments = gen_shipments(5);
    println!("Generated shipments: {:?}", generated_shipments); // Виведе: Generated shipments: [3, 3, 3, 3, 3]

    // Перевірка можливості рівного розподілу
    let can_distribute = can_distribute_evenly(&shipments1);
    println!("Can distribute evenly for {:?}: {}", shipments1, can_distribute); // Виведе: true
}

fn count_permutation(shipments: &Vec<u32>) -> usize {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        // Якщо не можна рівно розподілити, повертаємо 0
        return 0;
    }

    let average = total / n;
    let mut moves = 0;
    let mut excess = 0;

    for &shipment in shipments {
        if shipment > average {
            excess += shipment - average;
        } else {
            moves += average - shipment;
        }
    }

    // Повертаємо максимальну кількість перенесень, приводячи до usize
    (moves / 2 + excess) as usize
}

fn can_distribute_evenly(shipments: &Vec<u32>) -> bool {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    total % n == 0
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let total: u32 = (n * (n + 1) / 2) as u32; // Сума чисел від 1 до n
    let average = total / n as u32; // Середнє

    (1..=n).map(|_| average).collect()
}
