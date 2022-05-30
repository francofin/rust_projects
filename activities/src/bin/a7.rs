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


enum Color {
    Blue, 
    Brown,
    Yellow,
}

fn print_color(color: Color){
    let col = color;
    match col {
        Color::Blue => println!("Blue"),
        Color::Brown => println!("Brown"),
        Color::Yellow => println!("Yellow")
    }
}

fn main() {
    let col = Color::Blue;
    print_color(col);
}
