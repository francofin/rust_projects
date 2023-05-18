

//We can associate data with the variant in the enum. 
//When the function implementation is in the trait, it is available by default on the struct or enum that implements it. 
//It is not available on the instance unless defined in the implementation 
//This creates a associated funtion but if we want it on the instanc ebe sure to apply self to the function., 


// trait Log {
//     fn send_shout_out(&self);

//     fn display_info(&self);

//     fn alert(&self) {
//         println!("This is the default mesaage!!!!");
//     }
// }


// #[derive(Debug)]
// enum PersonId {
//     Passport(String),
//     IdentityCard(String),
// }

// struct Person{
//     name:String,
//     age: i32,
//     profession:String,
//     id: PersonId,
// }


// impl Log for Person {
//     fn send_shout_out(&self) {
//         println!("{} Says Hello", self.name);
//     }


//     fn display_info(&self){
//         println!("{} is {} years old and is a {}, identified by {}", self.name, self.age, self.profession, print_id(&self.id));
//         self.display_age();
//     }
    
    
// }

// impl Person {
//     fn new() -> Self{
//         Self {
//             name:"default".to_owned(),
//             age: 0,
//             profession: "default".to_owned(),
//             id:PersonId::Passport("Valid Passport".to_owned()),        }
//     }

//     fn from(name: String, age:i32, profession:String, id:PersonId) -> Person{
//         Person{
//             name,
//             age,
//             profession,
//             id,
//         }
//     }

//     fn display_age(&self){
//         //note in background rust is actually doing self: &Self
//         //this is a method
//         println!("{}", self.age);
//     }

//     fn change_age(&mut self, new_age: i32){
//         self.age = new_age;
//     }


//     // fn display_info(&self){
//     //     println!("{} is {} years old and is a {}, identified by {}", self.name, self.age, self.profession, print_id(&self.id));
//     //     self.display_age();
//     // }
// }

// fn print_id(id: &PersonId) -> String {

//     if let PersonId::Passport(value) = id {
//         println!("Matching Passport: {}", value);
//     } else {
//         println!("It does not match a Passport number");
//     }

//     let person_id = match id {
//         PersonId::Passport(value) => value,
//         PersonId::IdentityCard(value) => value,
//     };

//     person_id.to_string()

// }

// fn log_info(value: impl Log) {
//     value.display_info();
//     value.send_shout_out();
// }

// fn log_info2(value: &dyn Log) {
//     value.display_info();
//     value.send_shout_out();
//     value.alert();
// }


use rust_basics::rust_types::*;

fn main() {

    let another_person = Person::new();
    another_person.display_info();

    let mut person = Person::from("Vini Jr".to_owned(), 24, "Genius Footballer".to_owned(),PersonId::Passport("XHG77992".to_owned()));

    // person.set_name("Vini Jr".to_owned());
    
    person.change_age(22);
    person.send_shout_out();
    person.display_info();
    log_info(&person);

    person.set_name("Rodrygo Goes".to_owned());
    person.change_age(22);
    person.send_shout_out();
    person.display_info();
    log_info(&person);
    // Person::alert();
    // person.display_info();
    // println!("Person is {} and is {} who is a {}, identified by his {:?}",person.name, person.age, person.profession, print_id(person.id));
    let person3 = Person::from(
        String::from("John"),
        35,
        String::from("Lancaster"),
        PersonId::IdentityCard("ASDsu22378292Wer".to_owned()),
    );

    person3.display_info();
    log_info2(&person3);
    
    // println!("Person 3 is {} and is {} who is a {}, identified by his {:?}",person3.name, person3.age, person3.profession, print_id(person3.id));
}
