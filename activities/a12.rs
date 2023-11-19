// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color{
    Red,
    Black,
    Brown,
}

impl Color {
    fn print(&self) {
        match self {
            Color::Red => println!("Red"),
            Color::Black => println!("Black"),
            Color::Brown => println!("Brown"),
        }
    } 
}
struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}

struct Dimensions {
    height: f64,
    width: f64,
    depth: f64,
}

impl Dimensions{
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

impl Box {
    fn new(dimensions:Dimensions, weight: f64, color:Color) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print_box(&self){
        self.dimensions.print();
        self.color.print();
        println!("the weight is {:?}", self.weight);
    }
}

fn main() {
    let medium_dimensions = Dimensions{
        height: 10.1,
        width: 5.25,
        depth: 3.20,
    };

    let medium_box = Box::new(medium_dimensions, 5.0, Color::Red);
    medium_box.print_box();
}
