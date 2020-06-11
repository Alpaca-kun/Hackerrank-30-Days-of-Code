use std::io::*;

fn main() {
    let mut input_n = String::new();
    stdin().read_line(&mut input_n).unwrap();

    let input_n: usize = input_n.trim().parse().unwrap();
    for i in 1..=10 {
        println!("{} x {} = {}", input_n, i, input_n * i);
    }
}
