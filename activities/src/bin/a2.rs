// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result


fn add_num(a: i128, b:i128) -> i128{
    a+b
}

fn display_result(num: i128) {
    println!("{:?}", num)
}
fn main() {
    let my_sum = add_num(8,500);
    display_result(my_sum);
}
