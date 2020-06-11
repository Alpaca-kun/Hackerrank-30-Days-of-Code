use std::io::*;

fn factorial(x: usize) -> usize {
    if x <= 1 {
        1
    } else {
        x * factorial(x - 1)
    }
}

fn main() {
    let mut input_n = String::new();
    stdin().read_line(&mut input_n).unwrap();

    let int_n: usize = input_n.trim().parse().unwrap();
    println!("{}", factorial(int_n));
}
