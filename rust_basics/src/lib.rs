pub trait Log {
    fn send_shout_out(&self);

    fn display_info(&self);

    fn alert(&self) {
        println!("This is the default mesaage!!!!");
    }
}


#[derive(Debug)]
pub enum PersonId {
    Passport(String),
    IdentityCard(String),
}

pub struct Person{
    pub name:String,
    pub age: i32,
    pub profession:String,
    pub id: PersonId,
}


impl Log for Person {
    fn send_shout_out(&self) {
        println!("{} Says Hello", self.name);
    }


    fn display_info(&self){
        println!("{} is {} years old and is a {}, identified by {}", self.name, self.age, self.profession, print_id(&self.id));
        self.display_age();
    }
    
    
}

impl Person {
    pub fn new() -> Self{
        Self {
            name:"default".to_owned(),
            age: 0,
            profession: "default".to_owned(),
            id:PersonId::Passport("Valid Passport".to_owned()),        }
    }

    pub fn from(name: String, age:i32, profession:String, id:PersonId) -> Person{
        Person{
            name,
            age,
            profession,
            id,
        }
    }

    pub fn display_age(&self){
        //note in background rust is actually doing self: &Self
        //this is a method
        println!("{}", self.age);
    }

    pub fn change_age(&mut self, new_age: i32){
        self.age = new_age;
    }


    // fn display_info(&self){
    //     println!("{} is {} years old and is a {}, identified by {}", self.name, self.age, self.profession, print_id(&self.id));
    //     self.display_age();
    // }
}

pub fn print_id(id: &PersonId) -> String {

    if let PersonId::Passport(value) = id {
        println!("Matching Passport: {}", value);
    } else {
        println!("It does not match a Passport number");
    }

    let person_id = match id {
        PersonId::Passport(value) => value,
        PersonId::IdentityCard(value) => value,
    };

    person_id.to_string()

}


pub fn log_info(value: impl Log) {
    value.display_info();
    value.send_shout_out();
}

pub fn log_info2(value: &dyn Log) {
    value.display_info();
    value.send_shout_out();
    value.alert();
}