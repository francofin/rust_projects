// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait Perimeter{
    fn calculate_perimeter(&self);
}

struct Square{
    side: i32,
}
impl Perimeter for Square{
    fn calculate_perimeter(&self){
        let perimeter_calculated = self.side*4;
        println!("{:?}", perimeter_calculated)
    }
}


struct Triangle{
    a: i32,
    b: i32,
    c:i32,
}
impl Perimeter for Triangle{
    fn calculate_perimeter(&self){
        let perimeter_calculated = self.a + self.b + self.c;
        println!("{:?}", perimeter_calculated)
    }
}

fn get_perimeter(border: impl Perimeter){
    border.calculate_perimeter();
}

fn main() {

    get_perimeter(Square {side:4});
    get_perimeter(Triangle {a:5, b:5, c:5});

}
