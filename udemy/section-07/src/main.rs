/*
Let's model a road trip!
 
Define a `start_trip` function that creates and returns
a String of "The plan is..."
 
Invoke the `start_trip` function in `main` and save its
return value to a `trip` variable.
 
We want to pass the String to three separate functions
that will mutate the String without transferring ownership.
 
Define a `visit_philadelphia` function that concatenates
the text "Philadephia" to the end of the String. Invoke
the function in `main`. Then, invoke `push_str` on the String
to concatenate the content " and " to the end. Mak sure to
include the spaces.
 
Define a `visit_new_york` function that concatenates the
text "New York" to the end of the String. Invoke the function
in `main`. Repeat the previous logic to concatenate " and "
to the end of the String.
 
Define a `visit_boston` function that concatenates the
text "Boston." to the end of the String. Invoke the function
in `main`. Concatenate a period to the end of the
String/sentence.
 
Define a `show_itinerary` function that will print out
the final version of the String. Find a way to do so
without transferring ownership.
 
Invoke `show_itinerary`. The final output should be:
 
"The plan is...Philadelphia and New York and Boston."
*/

fn start_trip() -> String {
    return String::from("The plan is...");
}

fn visit_philadelphia(mut trip: String) -> String {
    trip.push_str("Philadelphia");
    trip
}

fn visit_new_york(mut trip: String) -> String {
    trip.push_str("New York");
    trip
}

fn visit_boston(mut trip: String) -> String {
    trip.push_str("Boston");
    trip
}

fn show_itinerary(trip: &String) {
    println!("{trip}");
}

fn main() {
    let mut trip: String = start_trip();
    
    trip = visit_philadelphia(trip);
    trip.push_str(" and ");
    
    trip = visit_new_york(trip);
    trip.push_str(" and ");

    trip = visit_boston(trip);
    trip.push('.');

    show_itinerary(&trip);
}
