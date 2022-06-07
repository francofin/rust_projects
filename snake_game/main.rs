

fn main() {
    let mut message = "Hello World";
    println!("{}", message);


    message = "No Wonder rust is safe";
    println!("{}", message);

    let another_message = "Whats up".to_owned();

    let my_new_message = print_welcome(another_message);

    println!("{}", my_new_message);
    

}




fn print_welcome(mut my_message: String) -> String{
    // println!("{}", my_message);
    let new_message_returned = my_message + " No Worries mon";
    // my_message.push_str(" No Worries Mon");
    //Can also use my_message/push_str(" No Worries Mon")

    new_message_returned
}