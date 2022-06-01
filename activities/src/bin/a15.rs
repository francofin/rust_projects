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
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info


#[derive(Debug, Clone)]
enum TicketType{
    Backstage(String, i32),
    Vip(String, i32),
    Standard(i32),
}


fn main() {
    let ticket_one = TicketType::Backstage("Fancois".to_owned(), 500);
    let ticket_two = TicketType::Vip("Julia".to_owned(), 800);
    let ticket_three = TicketType::Standard(200);

    let my_vector: Vec<TicketType> = vec![ticket_one, ticket_two, ticket_three];
    // println!("{:?}", &ticket_three);
    for ticket in my_vector{
        match ticket {
            TicketType::Backstage(name, amount ) => println!("{:?} Bought ticket for {:?} dollars", name, amount),
            TicketType::Vip(name, amount ) => println!("{:?} Bought ticket for {:?} dollars", name, amount),
            TicketType::Standard(amount ) => println!("Standard ticket costs {:?} dollars", amount),
        }

        
    }
    
}
