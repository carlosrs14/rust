fn main() {
    let food: &str = "pasta";
    let mut another: String = String::from(food);
    
    another.push_str(" in another");
    
    let mut change: String = String::clone(&another);

    drop(change);
    change = another.clone();
    
    println!("food are: {} and {} change is: {}", food, another, change);

}
