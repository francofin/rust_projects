

fn main() {
    let mut message = "Hello World".to_owned();
    println!("{}", message);

    let mut message_ref = message.clone();

    message_ref.push_str("From Franco");
    println!("Msg ref {}", message_ref);
    println!("original {}", message);
    


    // message = "No Wonder rust is safe";
    // println!("{}", message);

    let another_message = "Whats up".to_owned();

    let my_new_message = print_welcome(another_message);

    println!("{}", my_new_message);

    let mut new_message = String::from("Hello ");
    let mut crazy_message = "Franco ";

    new_message.push_str("World");
    println!("{}", new_message);

    let the_crazy_message = crazy_message.to_owned() + "Jack";
    println!("{}", the_crazy_message);

    let slice = &new_message[2..=4];

    let clone = new_message.clone();
    
    println!("{}", &new_message[2..4]);
    println!("{}", &new_message[2..=4]);
    println!("{}", &new_message[..]);
    println!("{}", clone);

}




fn print_welcome(mut my_message: String) -> String{
    // println!("{}", my_message);
    let new_message_returned = my_message + " No Worries mon";
    // my_message.push_str(" No Worries Mon");
    //Can also use my_message/push_str(" No Worries Mon")

    new_message_returned
}