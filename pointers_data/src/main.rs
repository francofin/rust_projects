use std::rc::Rc;
use std::cell::RefCell;

struct Flagger{
    is_true: RefCell<bool>
}

fn main() {
    

    let tup = (12, "eggs");
    let b = Box::new(tup);

    println!("{:?}", b);

    let x = 5;
    let y = &x;
    let z = &x;

    println!("{:?}", *z);

    let s1 = Rc::new(String::from("Pointer"));
    let s2 = s1.clone();
    let s3 = s2.clone();


    let flag = Flagger{is_true: RefCell::new(true)};
    let refr  = flag.is_true.borrow();


    println!("{:?}", refr);


    //cloning the value creates another value and adds a count to it. 

    //deallocation gives us the value at the memory address
}
