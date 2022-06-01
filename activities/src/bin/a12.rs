// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics


enum Color {
    Brown,
    Black,
    Yellow,
    Blue,
}

struct Dimensions {
    height: f64, 
    width: f64,
    depth: f64,
}

struct Box {
    weight: f64,
    dimensions:Dimensions,
    color: Color,
}

impl Box {

    fn new(weight:f64, color:Color, dimensions:Dimensions) -> Self {
        Self {
            weight: 0.0,
            dimensions,
            color:Color::Blue,
        }
    }

    fn print_color(&self) {
        match &self.color{
            Color::Brown => println!("Brown"),
            Color::Black => println!("Black"),
            Color::Yellow => println!("Yellow"),
            Color::Blue => println!("Blue"),
        }
    }

    fn print_char(&self) {
        println!("Box height is {:?} m", self.dimensions.height);
        println!("Box width is {:?} m", self.dimensions.width);
        println!("Box depth is {:?} m", self.dimensions.depth);
        println!("Box weight is {:?} lbs", self.weight);
        self.print_color();
    }
}

fn main() {

    let box_dimensions = Dimensions{
        width:30.8,
        height:10.5,
        depth:6.8,
    };

    let default_dimensions = Dimensions{
        width:0.0,
        height:0.0,
        depth:0.0,
    };

    let my_box = Box {
        weight: 200.69,
        dimensions: box_dimensions,
        color: Color::Yellow,
    };

    my_box.print_char();

    let default = Box::new(0.0, Color::Blue, default_dimensions);

    default.print_char();
}
