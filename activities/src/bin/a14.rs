// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function


struct Person {
    age: i32,
    name: String,
    fav_color:String,
}

fn print(data: &str){
    println!("{:?}", data);
}

fn main() {
    let team = vec![
        Person {
            age:33,
            name: String::from("Francois"),
            fav_color:"Yellow".to_owned(),
        },
        Person {
            age:27,
            name: String::from("Michael"),
            fav_color:"Red".to_owned(),
        },
        Person {
            age:40,
            name: String::from("Liz"),
            fav_color:"Black".to_owned(),
        }
    ];

    for member in team{
        if member.name  == "Francois"{
            println!("{:?} is {:?} years old and his favorite color is {:?}", member.name, member.age, member.fav_color);
            print(&member.name);
        } else {
            break;
        }
    }
}
