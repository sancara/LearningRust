// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Blue => println!("Blue"),
            Color::Green => println!("Green"),
        };
    }
}
struct Person {
    age: i32,
    name: String, 
    favCol: Color,
}

impl Person {
    fn new(age:i32, name:&str, favCol:Color) -> Self{
        Self {
            age,
            name: name.to_string(),
            favCol,
        }
    }
    fn print(&self) {
        println!("My name is {:?}", self.name);
        self.favCol.print();
    }
}


fn main() {

    let persons = vec![Person::new(33, "Santiago", Color::Blue), Person::new(36, "Laura", Color::Green), Person::new(1, "Juana", Color::Red)];

    for person in persons{
        if person.name == "Santiago" {
            person.print();
            println!("  ")
        } else if person.name == "Laura" {
            println!("{:?}", person.age);
            println!("  ")
        } else {
            person.favCol.print()
        }
    };
}
