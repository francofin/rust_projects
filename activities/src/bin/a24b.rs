trait Fall{
    fn hit_ground(&self);
    fn get_back_up(&self);
}

struct Vase;
impl Fall for Vase{
    fn hit_ground(&self){
        println!("planks")
    }

    fn get_back_up(&self){
        println!("All Good")
    }
}


fn fall(thing: impl Fall){
    thing.hit_ground();
    thing.get_back_up();
}

enum Color{
    Red,
    Blue,
    Black,
}

fn main() {
    fall(Vase {});

    let maybe_user = Some("Jerry");
    match maybe_user{
        Some(user) => println!("Hi {:?}", user),
        None =>  println!("User Not Found"),
    }

    //If user exist  print data esle print nothing. 
    // match is best on if we need both cases.
    //if let only matches ona  specific thing.  


    if let Some(user) = maybe_user{
        println!("Hi {:?}", user);
    } else {
        println!("no User");
    }

    let red  = Color::Red;
    if let Color::Red = red {
        println!("Red Printed");
    } else {
        println!("Im cools");
    }


    let mut data = Some(3);
    //As long as there is some data the loop runs, if in the loop we set data to None, loop terminates. 
    while let Some(i) = data{
        println!("loop");
        data = None;
    }

    println!("done");


    let numbers = vec![1,2,3,4,5];
    let mut number_iter  = numbers.iter();
    while let Some(num) = number_iter.next(){
        //Next returns optional value as long as there are values to be learned.
        println!("{:?}", num);
    }
 

   

}