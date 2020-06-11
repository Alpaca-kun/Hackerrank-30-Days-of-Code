use std::io::*;
use std::cmp::*;

fn main() {
    let mut input_n = String::new();
    stdin().read_line(&mut input_n).unwrap();

    let input_n: u32 = input_n.trim().parse().unwrap();
    let mut updated_input_n: u32 = input_n;
    let mut max_consecutive: u32 = 0;

    while updated_input_n != 0 {
        let leading_zeros = updated_input_n.leading_zeros();
        updated_input_n = updated_input_n << leading_zeros;
        let consecutive_ones = (!updated_input_n).leading_zeros();
        
        max_consecutive = max(max_consecutive, consecutive_ones);
        updated_input_n = updated_input_n << consecutive_ones;
    }

    println!("{}", max_consecutive);
}
