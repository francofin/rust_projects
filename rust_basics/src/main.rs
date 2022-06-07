fn main() {
    println!("Hello, world!");

    let custom_num = 98_000;
    let hex_num = 0xfa;
    let bin_num = 0b0010_1011;
    let byte_num = b'A'; //converts to 0x41

    println!("{}", custom_num);
    println!("{}", hex_num);
    println!("{}", bin_num);
    println!("{}", byte_num);

    let tup: (i32, String, f64, &str) = (50, "Hello".to_string(), 20.6, "World" );
    let (a,b,c,d) = tup;
    println!("{}", tup.2);
    println!("{}", d);


    let x = [1,4,5,6]; //Arrays, regular, not vectors
    let i = [2; 6]; // [2,2,2,2,2,2] -> 6 2s


    let num1 = 10;
    let num2 = num1;
    let num3 = 15;
    let sum = add(num1, num2);

    println!("{}", sum);

}


fn add(x:u32, y:u32) -> u32 {
    let result = x+y;
    result
}
