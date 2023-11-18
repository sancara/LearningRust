// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn save_coords(x: f64 , y: f64) -> (f64, f64){
    
    return (x, y);
}

fn main() {

    let (x, y) = save_coords(1.54, 5.);

    if y > 5.00 {
        println!("greater than 5")
    } else if y < 5.00 {
        println!("less than 5")
    } else {
        println!("equal to 5")
    }

}
