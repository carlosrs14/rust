fn fahrenheit_to_celsius(f: f32) -> f32 {
    (f - 32.0) / (9.0 / 5.0)
}

fn main() {
    println!("5°F -> {:.2}°C", fahrenheit_to_celsius(5.0));
    println!("6.2°F -> {:.2}°C", fahrenheit_to_celsius(6.2));
    println!("0°F -> {:.2}°C", fahrenheit_to_celsius(0.0));
    println!("90°F -> {:.2}°C", fahrenheit_to_celsius(90.0));
    println!("-30°F -> {:.2}°C", fahrenheit_to_celsius(-30.0));
}
