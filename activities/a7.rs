// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print
enum Colors {
    Red,
    Blue,
    White
}

fn print_color(my_color:Colors){
    match my_color {
        Colors::Red => println!("Color is red"),
        Colors::Blue => println!("Color is blue"),
        Colors::White => println!("Color is other than red or blue")
    }
}

fn main() {

    let color = Colors::Red;

    print_color(color);
    
}
