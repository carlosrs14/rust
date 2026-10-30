use std::io;

fn main() {
    println!("enter a number: ");
    let mut number_str = String::new();

    io::stdin().read_line(&mut number_str)
        .expect("Error reading the line");

    let number: i32 = number_str.trim().parse()
        .expect("Invalid number");

    if number % 2 == 0 {
        println!("is even");
    } else {
        println!("is odd");
    }
}
