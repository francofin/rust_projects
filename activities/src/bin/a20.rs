// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)\


use std::io;

enum PowerState{
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}


impl PowerState{
    fn new(state: &str) -> Option<PowerState>{
        let state = state.trim().to_lowercase();
        //modifying a string slice as with to lower creates an owned string
        //as_str(turns an owned string to a string slice)
        match state.as_str() {
            "off" => Some(PowerState::Off),
            "sleep" => Some(PowerState::Sleep),
            "reboot" => Some(PowerState::Reboot),
            "shutdown" => Some(PowerState::Shutdown),
            "hibernate" => Some(PowerState::Hibernate),
            _ => None
        }
    }

}

fn print_power_action(state: PowerState){
    use PowerState::*;
    match state{
        Off => println!("Powering Off"),
        Sleep => println!("Going to Sleep"),
        Reboot => println!("Reboot"),
        Shutdown => println!("Shutting Down"),
        Hibernate => println!("Hibernating"),
    }
}


fn main() {
    let mut buffer = String::new();
    let user_input_status = io::stdin().read_line(&mut buffer);
    //isok function is defined on Results to check if ok.
    //radline returns the bytes of the thing read in.
    if user_input_status.is_ok(){
       match PowerState::new(&buffer){
            Some(state) => print_power_action(state),
            None =>println!("Invalid Power State"),
       }
    } else {
        println!("Error Reading Input");
    }
}
