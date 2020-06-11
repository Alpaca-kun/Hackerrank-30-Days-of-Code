#![allow(non_snake_case)]

struct Person {
    age: i32
}

impl Person {
    fn new(initialAge: i32) -> Person {
        let mut initial_Age = initialAge;

        if initialAge < 0 {
            println!("Age is not valid, setting age to 0.");
            initial_Age = 0;
        }

        Person { age: initial_Age, }
    }

    fn amIOld(&self) {
        if self.age < 13 {
            println!{"You are young."}
        } else if (self.age >= 13) && (self.age < 18) {
            println!{"You are a teenager."}
        } else {
            println!{"You are old."}
        } 
    }

    fn yearPasses(&mut self) {
        self.age += 1;
    }
}

fn main() {
    let times: i32 = read_line().trim().parse().unwrap();

    for _ in 0..times {
        let age = read_line().trim().parse().unwrap();
        let mut person = Person::new(age);

        person.amIOld();

        for _ in 0..3 {
            person.yearPasses();
        }

        person.amIOld();
        println!("");
    }
}

fn read_line() -> String {
    let mut  input = String::new();
    std::io::stdin().read_line(&mut input).expect("Could not read stdin!");
    return input;
}
