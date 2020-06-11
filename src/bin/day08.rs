use std::io::*;
use std::collections::HashMap;

fn main() {
    let mut input_n = String::new();
    stdin().read_line(&mut input_n);

    let input_n: usize = input_n.trim().parse().unwrap();
    let mut buffer = String::new();
    let mut phone_book = HashMap::new(); 

    for _ in 0..input_n {
        stdin().read_line(&mut buffer);

        let mut name_and_phone = buffer.trim().split_whitespace();
        phone_book.insert(name_and_phone.next().unwrap().to_owned(), name_and_phone.next().unwrap().to_owned());

        buffer.clear();
    }

    let mut names_to_verify = vec![];
    while stdin().read_line(&mut buffer).unwrap()  >  0 {
        let name: String = buffer.trim().to_string();
        names_to_verify.push(name);

        buffer.clear();
    }

    for name_i in names_to_verify {
        match phone_book.get(&name_i) {
            Some(phone_i) => println!("{}={}", name_i, phone_i),
            None => println!("Not found"),
        }
    }
}
