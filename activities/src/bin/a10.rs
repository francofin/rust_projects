// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

enum Access {
    Admin,
    Manager, 
    User, 
    Guest
}

// fn print_message(decision: Bool) {
//  //
// }

fn main() {
    let access_level = Access::Guest;
    let can_access_file = match access_level{
        Access::Admin => true,
        _ => false,
    };

    println!("{:?}", can_access_file);
}
