use std::io::*;

fn main() {
    let mut input_of_strings = String::new();                                                  
    let mut input_n = String::new();

    stdin().read_line(&mut input_n).unwrap();  
    let input_n: usize = input_n.trim().parse().unwrap();

    for _ in 0..input_n{
        stdin().read_line(&mut input_of_strings).unwrap();
    }

    let iterator_of_string = input_of_strings.split_whitespace();

    for word in iterator_of_string {
        let chars_in_even_indexes: String = word.chars().step_by(2).collect();
        let chars_in_odd_indexes: String = word.chars().skip(1).step_by(2).collect();

        println!("{} {}", chars_in_even_indexes, chars_in_odd_indexes);
    }
}

