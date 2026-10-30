use std::io;

fn main() {
    let mut num_1_str = String::new();
    let mut num_2_str = String::new();
    let mut num_3_str = String::new();

    io::stdin().read_line(&mut num_1_str).expect("Error reading the line");
    io::stdin().read_line(&mut num_2_str).expect("Error reading the line");
    io::stdin().read_line(&mut num_3_str).expect("Error reading the line");

    let num_1: i32 = num_1_str.trim().parse().expect("Invalid number");
    let num_2: i32 = num_2_str.trim().parse().expect("Invalid number");
    let num_3: i32 = num_3_str.trim().parse().expect("Invalid number");

    if num_1 > num_2 {
        if num_1 > num_3 {
            println!("num_1 is bigger");
        } else {
            println!("num 3 is bigger");
        }
    } else if num_2 > num_3 {
        println!("num 2 is bigger");
    }
}
