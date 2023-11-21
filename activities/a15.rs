// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
enum Ticket{
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String),
}

// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

fn main() {
    
    let concert_tickets = vec![
        Ticket::Standard(19.99), 
        Ticket::Backstage(50.00, "Santi".to_owned()), 
        Ticket::Vip(99.99, "Lauri".to_owned())
        ];

    for ticket in concert_tickets {
        match ticket {
            Ticket::Backstage(price, holder) => println!("Backstage ticket: holder is {holder}, price = {price}"),
            Ticket::Standard(price) => println!("Standar ticket price is {price}"),
            Ticket::Vip(price, holder) => println!("V.I.P. ticket: holder is {holder}, price = {price}"),
        }
    };
}
