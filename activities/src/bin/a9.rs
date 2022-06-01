// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn return_tuple_of_integers(a:i32, b:i32) -> (i32, i32){
    (a, b)
}

fn print_y_coord(num: i32){
    if num > 5{
        println!("y cord is greater than 5");
    } else if num < 5 {
        println!("y cord is less than 5");
    } else {
        println!("y cord is equeal to 5");
    }
}

fn main() {
    let (_x,y) = return_tuple_of_integers(10,20);
    print_y_coord(y);
}
