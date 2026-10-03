use std::any::type_name_of_val;

fn main() {
    let a = 10;
    let b = 20.5;
    let c = true;


    println!("{}", type_name_of_val(&a));
    println!("{}", type_name_of_val(&b));
    println!("{}", type_name_of_val(&c));
}
