use std::io::*;

fn main () {
    let mut input_n= String::new();
    stdin().read_line(&mut input_n);

    let input_n: u32 = input_n.trim().parse().unwrap();
    match input_n {
        2 | 4 => println!("Not Weird"),
        _ if (input_n > 20) && (input_n % 2 == 0) => println!("Not Weird"),
        _ => println!("Weird"),
    }
}
