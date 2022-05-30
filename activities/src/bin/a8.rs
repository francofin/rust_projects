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
//f 64 allows us to work with double. 


enum Flavor {
    Cola,
    Orange,
    Grape
}

struct Drink {
    drink_flavor: Flavor,
    oz:f64,
}


fn print_drink(my_drink: Drink) {
    match my_drink.drink_flavor{
        Flavor::Cola => println!("I bought a {:?} soda, Cola flavored", my_drink.oz),
        Flavor::Orange => println!("I bought a {:?} soda, Orange flavored", my_drink.oz),
        Flavor::Grape => println!("I bought a {:?} soda, Grape flavored", my_drink.oz),
    }
}

fn main() { 
    let my_item =  Drink {
        drink_flavor: Flavor::Orange,
        oz:2.05
    };

    print_drink(my_item);


}
