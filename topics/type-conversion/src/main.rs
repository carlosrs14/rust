fn main() {
    let a = 10 as f32;
    let b = 3.5;

    let decimal = a / b;
    let entire = (a / b) as i32;

    println!("decimal = {}", decimal);
    println!("entire = {}", entire);
}
