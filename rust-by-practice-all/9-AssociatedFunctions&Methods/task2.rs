#[derive(Debug)]
struct TrafficLight {
    color: String,
}

impl TrafficLight {
    pub fn show_state(&self)  {
        println!("the current state is {}", self.color);
    }
}

fn main() {
    let light = TrafficLight{
        color: "red".to_owned(),
    };
    light.show_state();
    println!("{:?}", light);
}
