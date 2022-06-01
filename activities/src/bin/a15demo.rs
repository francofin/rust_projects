
/// Creates an enum
enum Discount{
    Percent(i32),
    Flat(i32),
}

/// Creates a Struct 
struct Ticket{
    event: String,
    price: i32,
}


///Runs the Main Function
fn main() {
    let n = 3;
    match n {
        3 => println!("threee"),
        other => println!("number: {:?}", other),
        // Other is used to replaced the _ makes it easier to read.
    }

    let flat = Discount::Flat(2);
    match flat{
        Discount::Flat(amount) => println!("Flat discount of {:?}", amount),
        Discount::Flat(2) => println!("Your get 2% discount."),
        _ => (),
        //Set of () means we return nothign. 
    }

    let concert  = Ticket {
        event: "concert".to_owned(),
        price: 50,
    };

    //To match on structs we use {}
    //.. ignores everythings else in the struct. 

    match concert{
        Ticket {price, ..} => println!("price = {:?}", price), 
        // Ticket {price:50, event => println!("event at 50 = {:?}", event), 
    }
}