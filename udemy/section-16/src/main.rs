/*
Bring the HashMap type into the current's file's namespace.
 
Declare a `sauces_to_meals` HashMap. The keys will be
string slices and the values will be a vector of string
slices. Use the `from` function to populate the HashMap
with 2 key-value pairs:
 
Key: "Ketchup"
Value: Vector of ["French Fries", "Burgers", "Hot Dogs"]
 
Key: "Mayonnaise"
Value: Vector of ["Sandwiches", "Burgers", "Coleslaw"]
 
Use the `insert` method to add the following key-value
pair to the HashMap.
 
Key: "Mustard"
Value: Vector of ["Hot dog", "Burgers", "Pretzels"]
 
Use the `remove` method to remove the key-value pair
where "Mayonnaise" is the key. Find a way to retrieve
the vector inside the Option and print it out.
 
Use the `get` method to retrieve the key-value pair
where "Mustard" is the key. Find a way to retrieve
the vector inside the Option and print it out.
 
Use the `entry` and `or_insert` methods to add the
following key-value pair:
 
Key: "Soy Sauce"
Value: Vector of ["Sushi", "Dumplings"]
 
Finally, print out the final `sauces_to_meals` HashMap.
 
The final result should be:
{
  "Ketchup": ["French Fries", "Burgers", "Hot Dogs"],
  "Soy Sauce": ["Sushi", "Dumplings"],
  "Mustard": ["Hot dog", "Burgers", "Pretzels"]
}
*/

use std::collections::HashMap;

fn main() {
    let mut sauces_to_meal = HashMap::<String, Vec<String>>::from([
        (String::from("Ketchup"), vec![String::from("French Fries"), String::from("Burgers"), String::from("Hot Dogs")]), 
        (String::from("Mayonnaise"), vec![String::from("Sandwiches"), String::from("Burgers"), String::from("Coleslaw")])
    ]);

    sauces_to_meal.insert(
        String::from("Mustard"), 
        vec![String::from("Hot Dogs"), String::from("Burgers"), String::from("Pretzels")]);
    
    match sauces_to_meal.remove(&String::from("Mayonnaise")) {
        Some(data) => println!("{:?}", data),
        None => println!("No key named Mayonnaise")
    }

    match sauces_to_meal.get(&String::from("Mustard")) {
        Some(data) => println!("{:?}", data),
        None => println!("No key named Mustard")
    }

    let data = (String::from("Soy Sauce"), vec![String::from("Sushi"), String::from("Dumplings")]);
    sauces_to_meal.entry(data.0).or_insert(data.1);

    println!("{:?}", sauces_to_meal);
}
