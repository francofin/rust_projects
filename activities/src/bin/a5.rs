// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    let mut max_number = 0;
    loop {
        if max_number == 5 {
            break;
        }
        println!("{:?}", max_number);
        max_number += 1;
    }
}
