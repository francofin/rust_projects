// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

use std::io;

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn main() {
    println!("Please Enter The User You are interested in");
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please Enter Your input data again")
        
    }
    let input  = buffer.trim().to_owned();
    let user = find_user(&input).map(|query| User{user_id:query, name:input.to_owned()});
    // println!("{:?}", user);
    match user {
        Some(user) => println!("{:?}", user),
        None => println!("User Not Found"),
    }
}
