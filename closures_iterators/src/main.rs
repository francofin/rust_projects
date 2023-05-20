
#[derive(Debug)]
struct City{
    city: String,
    population: u64
}


#[derive(Debug)]
struct Item{
    name: String,
}
// fn sort_pop(city: &mut Vec<City>){
//     city.sort_by_key(pop_helper)
// }

// fn pop_helper(pop: &City) -> u64{
//     pop.population
// }

#[derive(Debug)]
struct Range{
    start: u64,
    end:u64
}

impl Iterator for Range{
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item>{
        if self.start >= self.end{
            return None;
        }
        let result = Some(self.start);
        self.start +=1;
        result
    }
}

//Takes ownbership of the vector. 
fn check_inventory(items: Vec<Item>, product: String) -> Vec<Item>{
    items.into_iter().filter(|i| i.name ==product).collect() //return back to vec using collect. 
}

fn sort_pop_closure(pop: &mut Vec<City>){
    pop.sort_by_key(|p|p.population); //p is arg in closure
}
fn main() {
    let city_one = City{city:String::from("A"), population:100};
    let city_two = City{city:String::from("B"), population:80};
    let city_three = City{city:String::from("C"), population:1000};
    let city_four = City{city:String::from("D"), population:10};
    let city_five = City{city:String::from("E"), population:1980};

    let mut city_vec: Vec<City> = Vec::new();
    city_vec.push(city_one);
    city_vec.push(city_two);
    city_vec.push(city_three);
    city_vec.push(city_four);
    city_vec.push(city_five);


    sort_pop_closure(&mut city_vec);

    println!("{:?}", city_vec);

    let add = |x: i32| -> i32 {x+1};
    let add_two = |x| x+2;
    add_two(1);


    let vec = vec![1,2,3];

    for val in vec.iter(){
        println!("{}", val);
    }

    let vec2 = vec![1,2,3];
    let mut iter = (&vec2).into_iter();

    while let Some(v) = iter.next(){
        println!("{}", v);
    }

    let mut item_vec: Vec<Item> = Vec::new();
    item_vec.push(Item{name:String::from("Shirt")});
    item_vec.push(Item{name:String::from("Short")});
    item_vec.push(Item{name:String::from("Shoes")});
    item_vec.push(Item{name:String::from("Skirt")});
    item_vec.push(Item{name:String::from("Pants")});

    let checked = check_inventory(item_vec, String::from("Shirt"));
    println!("{:?}", checked);

    let mut vec_cust = Range{start:0, end:10};
    for r in vec_cust{
        println!("{:?}", r);
    }
    let mut vec_mod: Vec<u64> = Range{start:0, end:10}.filter(|x| x%2 ==0).collect();
    println!("{:?}", vec_mod);
}

// trait Iterator{
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }