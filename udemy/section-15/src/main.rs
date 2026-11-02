/*
Define a `make_money` function that accepts a mutable
String reference. The function should concatenate
the characters "$$$" to the end of the String.
Invoke the function in `main`.
 
Define a `trim_and_capitalize` function that accepts
a string slice. It should return a String with
all whitespace removed and all characters in uppercase.
Invoke the function in `main`.
 
Define an `elements` function that accepts a string
slice. It should split the string by all occurrences
of the `!` symbol and return a vector of the string
slices. Invoke the function in `main`.
 
Example:
elements("Gold!Silver!Platinum")
=> Vector of ["Gold", "Silver", "Platinum"]
 
Define a `get_identity` function. The function should
ask the user for their first and last name in TWO
steps (i.e., collect user input twice). Make sure
to communicate the instructions to the user.
For each Result enum you receive, call the `expect`
method and provide a custom error message (like
"Failed to collect first name"). Return a String
with the first and last names combined. Invoke
the `get_identity` function in `main`, and output the
returned String.
 
Example:
fn main() {
  let name = get_identity();
   println!("{name}"); // Bill Murray
}
*/

use std::io::Write;

fn make_money(s: &mut String) {
    s.push_str("$$$");
}

fn trim_and_capitalize(s: &str) -> String {
    return String::from(s.trim().to_uppercase());
}

fn elements(s: &str) -> Vec<&str> {
    return s.split('!').collect();
}

fn get_identity() -> String {
    let mut firstname = String::new();

    print!("Insert your firstname: ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut firstname)
        .expect("Can not read the name");


    let mut lastname = String::new();

    print!("Insert your lastname: ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut lastname)
        .expect("Can not read the name");
 
    return format!("{} {}", firstname.trim(), lastname.trim());
}

fn main() {
    let mut s = String::from("Hello, world!");
    make_money(&mut s);
    println!("{}", s);

    println!("{}", trim_and_capitalize(&s));

    println!("elements: {:?}", elements("Gold!Silver!Platinum"));

    println!("Full Name: {}", get_identity());
}
