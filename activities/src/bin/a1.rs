// Topic: Functions
//
// Program requirements:
// * Displays your first and last name
//
// Notes:
// * Use a function to display your first name
// * Use a function to display your last name
// * Use the println macro to display messages to the terminal


// 

fn display_name(name: String) {
    println!("My Name is {:?}", name);
}

fn sub(a: i32, b: i32) -> i32 {
    a - b
}

fn main() {
    let a = "Francois";
    display_name(a.to_string());

    let sum = 2+2;
    let value = 10-5;

    let my_var = sub(sum, value);
    println!("{:?}", my_var);
}
