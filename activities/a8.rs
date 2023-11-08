// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Galletitas{
    Bagley,
    Terrabusi,
    Cerealitas,
}

struct Galles{
    tipo: Galletitas,
    stock: i32,
    precio: i32,
}

fn print_galles(g: Galles) {
    match g.tipo {
        Galletitas::Bagley => println!("Galles: Bagley"),
        Galletitas::Terrabusi => println!("Galles: Terrabusi"),
        Galletitas::Cerealitas => println!("Galles: Cerealitas"),
    }

    println!("Te compraste {:?} paquetes y el precio es {:?}", g.stock, g.precio * g.stock);
}

enum DrinkFlavor {
    Caipirinha,
    Gin,
    Fernet,
}

struct DrinkInfo {
    drink_flavor:DrinkFlavor,
    ounce: f64,
}

fn print_drink(drink: DrinkInfo){
    match drink.drink_flavor {
        DrinkFlavor::Caipirinha => println!("Your Drink is a Caipi"),
        DrinkFlavor::Gin => println!("Your Drink is a Gin Tonic"),
        DrinkFlavor::Fernet => println!("Your Drink is a Fernet"),
    }

    println!("It has {:?} ounces", drink.ounce)
}

fn main() {

    let my_drink = DrinkInfo{
        drink_flavor: DrinkFlavor::Gin,
        ounce: 0.45,
    };

    print_drink(my_drink);

    let galle = Galles{
        tipo: Galletitas::Cerealitas,
        stock: 2,
        precio: 180,
    };

    print_galles(galle);
}
