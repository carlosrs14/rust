extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("The magic number");

    let random_num = rand::thread_rng().gen_range(1, 101);

    let mut user_input = String::new();
    let mut user_num: i32;
    
    loop {
        println!("Your guess: ");

        user_input.clear();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Could not read line");

        user_num = user_input
            .trim()
            .parse::<i32>()
            .expect("Invalid number");

        match user_num.cmp(&random_num) {
            Ordering::Equal => {
                println!("You won");
                break;
            }
            Ordering::Greater => println!("Your number is greater"),
            Ordering::Less => println!("Your number is smaller")
        }
    }
}
