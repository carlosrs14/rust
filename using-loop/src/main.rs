fn main() {
    let mut seconds: i32 = 10;

    loop {
        if seconds == 0 {
            println!("Boom");
            break;
        }

        println!("{} seconds", seconds);
        seconds -= 1;
    }
}
