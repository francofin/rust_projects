
pub mod group_one;
pub mod helper;


pub fn print_from_lib() {
    use helper::{print_from_helper, print_again};
    println!("Printed from Lib");
    helper::print_from_helper();
    print_from_helper();
    print_again();
    group_one::g1::print_from_g1();
}