use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_indices = (0, 1);
    
    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indices = (i, i + 1);
        }
    }
    
    (min_sum, min_indices.0, min_indices.1)
}

fn print_vector(data: &[i32], min_info: (i32, usize, usize)) {
    let (min_sum, idx1, idx2) = min_info;

    // Виводимо індекси
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:2}.", i)).collect::<String>());
    // Виводимо дані
    println!("data:    {:?}", data);
    // Виводимо позиції мінімальної пари
    println!("indexes: {}", (0..data.len()).map(|i| if i == idx1 || i == idx2 { "\\__" } else { "    " }).collect::<String>());
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[idx1], data[idx2], min_sum, idx1, idx2);
}

fn main() {
    let n = 20;
    
    for _ in 0..4 { // Для кількох прикладів
        let data = gen_random_vector(n);
        let min_info = min_adjacent_sum(&data);
        print_vector(&data, min_info);
        println!(); // Розділення між прикладами
    }
}
