


fn main() {
//Loop key word
    // loop {
    //     println!("{}", "loop");
    // }

    let mut while_count = 0;
    while while_count < 5 {
        println!("{}", while_count);
        while_count+=1;
    }

    let mut count = 0;
    
    'counter: loop{
        println!("{}", count);
        let mut decrease = 5;
        loop {
            println!("Decreasing {}", decrease);
            if decrease == 4 {
                break;
            }
            if count == 2{
                break 'counter;
            }

            decrease -=1;
        }
        count+=1;
    }

   let decimal = 02_55;
   let hex = 0xfff;
   let octal = 0o322;
   let byte = b'A';

   println!("{}", byte);

    let tup  = (0, 255, "Hello", true);

    println!("{}", tup.2);

    let mut nums = vec![1,2,3];
    nums.push(4);

    println!("{:?}", nums);

    nums.reverse();

    println!("{:?}", nums);

    let mut vect = Vec::<i32>::with_capacity(2);
    println!("{:?}", vect.capacity());

    let mut vect_two: Vec<i32> = (0..5).collect();
    println!("{:?}", vect_two);

    for el in &vect_two{
        println!("{}", el);
    }

    let v: &[i32] = &vect_two[2..4];
    println!("{:?}", v);

    let name = String::from("Francois");
    let new_name = "Kieran".to_string();

    let new_first_name = name.replace("Francois", "Carlos");

    let string_slice = "My Striong Slize"; //String slice
    let str_slice_two = string_slice.to_string(); //String
    let str_slice_three = &str_slice_two; //string slice

    println!("{:?}", name);
    println!("{:?}", new_name);
    println!("{:?}", new_first_name);


    //String literal
    let rust  = "\x52\x75\x73\x74";
    println!("{:?}", rust);

    println!("Hello, world!");

    let gcd = greatest_deonominator(20, 40);
    println!("{}", gcd);
}


fn print_phrase(phrase: &str){
    println!("{:?}", phrase);
}

fn greatest_deonominator(mut x: u64, mut y: u64) -> u64 {
    while x != 0 {
        if x < y{
            let z = x;
            x= y;
            y= z;
        }
        x = x % y;
    }
    y
}

