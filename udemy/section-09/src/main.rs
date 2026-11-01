/*
Define a Flight struct with the following fields:
  - an `origin` field (String)
  - a `destination` field (String)
  - a `price` field (f64)
  - a `passengers` field (u32)
 
Derive a Debug trait implementation for the Flight struct.
 
Define a `new` constructor function that returns a new
instance of a Flight.
 
Define a `change_destination` method that accepts a new
destination and overwrites the value of the `destination`
field.
 
Define a `increase_price` method that raises the value
of the `price` by 20% (multiply the `price` field by 1.20).
Make sure to save the new `price` field value.
 
Define a `itinerary` method that prints out both the
`origin` and `destination` fields in the following format
(origin -> destination).
 
Use the constructor function to create a new Flight instance
in the main function. Invoke all of the defined methods.
Print out the struct in Debug format to confirm the struct
updates as you expect.
 
Use struct update syntax to copy the `price` and `passengers`
fields to a new Flight struct instance. Make sure to provide
new Strings for the remaining fields to ensure ownership
doesn't transfer. Assign the new Flight to a separate variable.
*/

#[derive(Debug)]
struct Flight {
    origin: String,
    destination: String,
    price: f64,
    passengers: i32
}

impl Flight {
    fn new(origin: String, destination: String, price: f64, passengers: i32) -> Flight {
        return Flight {
            origin,
            destination,
            price,
            passengers
        };
    }
    
    fn change_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    fn increase_price(&mut self) {
        self.price = self.price * 1.2;
    }

    fn itinerary(&self) {
        println!("({} -> {})", self.origin, self.destination);
    }

    fn copy(&self, origin: String, destination: String) -> Flight {
        return  Flight {
            origin,
            destination,
            .. *self
        };
    }
}

fn main() {
    let origin = String::from("origin");
    let destination = String::from("destination");
    let price = 4999.99;
    let passengers = 50;

    let mut flight = Flight::new(origin , destination, price, passengers);

    println!("{:?}", flight);
    flight.change_destination(String::from("another destination"));
    flight.increase_price();

    flight.itinerary();
    println!("{:?}", flight);

    let another_flight = flight.copy(String::from("origin copied"), String::from("destination copied"));

    println!("{:?}", another_flight);
    println!("{:?}", flight);
}
