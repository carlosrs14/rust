fn main() {
    let season: &str = "wintr";
    match season {
        "winter" => {
            println!("its winter");
        },
        "spring" => {
            println!("all rigth");
        },
        _ => {
            println!("invalid");
        }
    }
}