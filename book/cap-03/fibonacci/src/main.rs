fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}
fn main() {
    println!("Fibonacci 1: {}", fibonacci(1));
    println!("Fibonacci 5: {}", fibonacci(5));
    println!("Fibonacci 6: {}", fibonacci(6));
    println!("Fibonacci 7: {}", fibonacci(7));
    println!("Fibonacci 16: {}", fibonacci(16));
}
