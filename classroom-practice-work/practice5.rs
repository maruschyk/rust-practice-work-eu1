const WIDTH: usize = 10;
const HEIGHT: usize = 5;

fn main() {
    // Верхня частина конверта
    for _ in 0..WIDTH {
        print!("=");
    }
    println!();

    // Середина конверта
    for _ in 0..HEIGHT-2 {
        print!("|");
        for _ in 1..WIDTH-1 {
            print!(" ");
        }
        println!("|");
    }

    // Нижня частина конверта (з клапаном)
    for i in 0..WIDTH {
        if i < WIDTH/2 {
            print!("=");
        } else {
            print!("/");
        }
    }
    println!();
}
