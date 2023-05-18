use std::ops::Add;

#[derive(Debug)]
struct Point<T, U> {
    x: T, 
    y: U
}

#[derive(Debug)]
struct Coord<T> {
    x: T, 
    y: T
}

impl<T> Add for Coord<T>
where //restricts T to types that can add to themselves to yield another T value
T: Add<Output = T>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self{
        Coord{
            x: self.x + rhs.x, //lfs
            y: self.y + rhs.y// rhs

        }
    }
}

trait OverView {
    fn overview(&self) -> String{
        String::from("This is a Rust Course")
    }
}

// trait Clone: Sized {
//     fn clone(&self) -> self;
//     fn clone_from(&mut self, source: & self){
//         *self = source.clone()
//     }
// }

struct Course{
    headline: String, 
    author: String,
}

struct Lecture{
    headline: String,
    author: String
}

impl OverView for Course{
    fn overview(&self) -> String{
        format!("{}, {}", self.author, self.headline)
    }
}

impl Drop for Course{
    fn drop(&mut self){
        println!("Dropping, {}", self.author)
    }
}

// impl OverView for Lecture{
//     fn overview(&self) -> String{
//         format!("{}, {}", self.author, self.headline)
//     }
// }


impl OverView for Lecture{}

fn main() {


    let coord_one = Coord{x: 5.0, y:5.0};
    let coord_two = Coord{x: 1.0, y:51.0};

    let coord_sum = coord_one + coord_two;

    println!("{:?}", coord_sum);
    let point_one = Point{x: 50.0, y: 550.0};
    let point_two = Point{x: 'x', y: 550.0};
    println!("Hello, world!");
    println!("{:?}", point_one);
    println!("{:?}", point_two);

    let course_one = Course{headline: String::from("Headline"), author: String::from("Francois")};
    let lecture_one = Lecture{headline: String::from("Lecture"), author: String::from("Francois j")};

    println!("{:?}", course_one.overview());
    println!("{:?}", lecture_one.overview());

    call_overview(&course_one);
    call_overview(&lecture_one);

    // drop(course_one);
}

//These 2 are the same.

// fn call_overview(item: &impl OverView){
//     println!("Overview {}", item.overview())
// }

fn call_overview<T: OverView>(item: &T){
    println!("Overview {}", item.overview())
}

// fn call_overview(item1: &impl OverView, item2: &impl OverView) can be different types
// fn call_overview<T: OverView>(item1: &T, item2: &T) must be same type
// fn call_overview(item1: &impl OverView + another trait)  multiple trait bounds
// fn call_overview<T: OverView + anotherTrait>(item1: &T, item2: &T) must be same type