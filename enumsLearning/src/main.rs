enum Pet{
    dog, 
    cat, 
    fish
}

impl Pet {
    fn what_am_i(&self) ->&'static str {
        match self{
            Pet::dog => "I am a dog",
            Pet::cat => "I am a cat",
            Pet::fish => "I am a fish",

        }
    }
}

#[derive(Debug)]
enum IPAddrKind{
    V4,
    V6
}


#[derive(Debug)]
struct IPAddr{
    kind: IPAddrKind,
    address: String,
}


fn main() {
    let dog  = Pet::dog;
    println!("{:?}", dog.what_am_i());

    let home = IPAddr{kind:IPAddrKind::V6, address: String::from("192.168.0.1")};

    println!("{:?}", home);

    let some_number = Some(5);
    let some_string = Some("a string");
    let nothing: Option<i32> = None;

    let x: Option<i32> = Some(5);
    let y: i32 = 5;

    let five = Some(5);
    let six = plus_one(five);

    let none = plus_one(None);

    println!("{:?}", six);
    println!("{:?}", none);

    let dog2 = Some(Pet::dog);
    if let Some(Pet::dog) = dog2{
        println!("Animal is a dog")
    } else {
        println!("Animal is not a dog");
    }
    
    let mut stack  = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top)= stack.pop() {
        println!("{}", top);
    }

    let new_x = 5;
    match new_x {
        1 | 2 => println!("1 or 2"),
        _ => println!("Not 1 or 2"),
    }

    match new_x{
        1..=9 => println!("Matches"),
        _ => println!("Not Matching")
    }

    let another_x = Some(5);
    let another_y = 5;

    match another_x {
        Some(10) => println!("Its 10"),
        Some(another_x) if another_x == another_y => println!("Matches"), //another_x is actually just 5 in this statement
        _ => println!("No Matches")
    }
}


fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i+1),

    }
}

fn what_pet(input: &str) {
    match input{
        "Dog" => println!("I have a dog"),
        "Cat" => println!("I have a cat"),
        "Fish" => println!("I have a fish"),
        _ => println!("You have no pets"),
    }
}

//if let for match with one case and can have an else for unmathcing options. Can also use else if and etc. 