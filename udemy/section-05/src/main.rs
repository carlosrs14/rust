/*
Define a `color_to_number` function that accepts a 'color'
parameter (a string). Use if, else if, and else
statements to return a corresponding numeric value based
on the following rules:
1. If the color is "red", return 1.
2. If the color is "green", return 2.
3. If the color is "blue", return 3.
4. If the color is any other string, return 0.
 
Refactor the function above to use the `match` statement
instead of if, else if, and else.
 
Define a `factorial` function that calculates the
factorial of a number. The factorial is the product
of multiplying a number by every incremental
number leading up to it, starting from 1.
 
Examples:
The factorial of 5 is 5 * 4 * 3 * 2 * 1 = 120
factorial(5) should return 120.
 
The factorial of 4 is 4 * 3 * 2 * 1 = 24
factorial(4) should return 24.
 
Implement two solutions/functions for the problem.
The first solution should not use recursion.
The second solution should use recursion.
*/

fn color_to_number(color: &str) -> i8 {
    return match color {
        "red" => 1,
        "green" => 2,
        "blue" => 3,
        _ => 0
    }
}

fn factorial_rec(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    return n * factorial_rec(n - 1);
}
fn factorial_itr(n: i32) -> i32 {
    let mut result: i32 = 1;
    let mut i: i32 = 1;

    while i <= n {
        result *= i;

        i += 1;
    }
    return result;
}

fn main() {
    println!("{}", color_to_number("red"));
    println!("{}", color_to_number("green"));
    println!("{}", color_to_number("yellow"));

    println!("{}", factorial_rec(5));
    println!("{}", factorial_itr(5));
}
