// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem {
    id: i32,
    qty: f64,
}

fn display_qty(item: &GroceryItem) {
    println!("The quantity is: {:?}", item.qty)
}

fn display_id(item: &GroceryItem) {
    println!("The id number is: {:?}", item.id)
}

fn main() {

    let banana = GroceryItem {
        id: 1,
        qty: 1.25
    };

    display_qty(&banana);

    display_id(&banana);

}
