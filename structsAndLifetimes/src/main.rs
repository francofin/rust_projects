#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    sign_in_count: u32
}

//Lifetimes prevent dangling references

//Tuple struct
#[derive(Debug)]
struct Coordinates(i32, i32, i32);

struct Square {
    width: u32,
    height: u32
}

impl Square{
    fn area(&self) -> u32{
        self.width * self.height
    }

    fn perimeter(&self) -> u32{
        4*self.width
    }

    fn change_widht(&mut self, new_width: u32){
        self.width = new_width;
    }
}

struct MyString<'a> {
    text: &'a String,
}

//Unit like
//.. is short hand for range as in (0..5).collect()
fn main() {
    let my_string = String::from("My String");
    let x = MyString{text: &my_string};

    println!("Hello, world!");
    let user_one = User{active:true, username:"Mike".to_string(), sign_in_count:0};
    println!("{:?}", user_one);

    let user_two = create_user(String::from("Sarah"));
    println!("{:?}", user_two);

    let coords = Coordinates(223, 243, 211);
    println!("{:?}", coords);

    let mut sq = Square{width:5, height:5};
    println!("{}", sq.perimeter());
}


fn create_user(username: String) -> User{
    User{
        username,
        active: true,
        sign_in_count: 1
    }
}

//Lifetime of return must match lifetime of param. i.e. a to a.
//Static lifetimes live for dur of program. Stored in the binary. 

fn lifetime_example<'a, 'b>(x: &'a str, y: &'b str) -> &'b str{
    y
}