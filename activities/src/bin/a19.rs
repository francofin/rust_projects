// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock
use std::collections::HashMap;


fn main() {
    let mut stock = HashMap::new();
    stock.insert("Chai", 3);
    stock.insert("Tea", 10);
    stock.insert("Coffee", 4);
    stock.insert("Sugar", 6);
    stock.insert("Milk", 0);
    stock.insert("Cookies", 2);

    let mut total_stock = 0;


    for (item, quantity) in stock.iter(){
        total_stock+=quantity;
        match quantity{
            0 => println!("There is no {:?} in stock", item),
            _ => println!("There is a quantity of {:?} {:?} in stock", quantity, item),
        }


//Format saves a message into the variable rather than just print it out. 
        let stock_count = if quantity < &1 {
            "out of stock".to_owned()
        } else{
            format!("{:?}", quantity)
        };

        println!("There is a quantity of {:?} {:?} in stock", stock_count, item);
    }

    println!("--------------------------------------------------------------------------------");

    println!("There are {:?} items in the grocery", total_stock);
}
