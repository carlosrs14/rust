/*
println!
String
&str
i32
f32
bool
char
*/

fn main() {
    let name: &str = "Carlos";
    let age: i16 = 22;
    let heigth: f32 = 1.65;
    let is_student: bool = true;
    println!("Hi my name is {}, I'm {}", name, age);
    println!("My is {}, and is student = {}", heigth, is_student);

}
