fn main() {
    println!("Hello, world!");

    let mut x:f64 = 0.5554;
    println!("The value of x is {}", x);

    x = 0.55442;
    println!("The value of x is now {}", x);

    let (my_first_var, my_second_var) = (23,55);
    println!("The values are {:?}", (my_first_var, my_second_var))
}
