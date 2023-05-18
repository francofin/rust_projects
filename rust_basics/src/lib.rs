
//imoport using module with name of file. use keyword makes the pth shorter to access. mod is used for import. 

mod extension_lib;


use extension_lib::external_lib::*;

fn outsider() {
    println!("Printing Library loaded");
    extension_lib::external_lib::external_lib_activated();
}




pub mod rust_types {

    use std::fmt::*;


    //We can have modules within modules, but even they must be marked as public for use in rust. 
    //We can use the crate key word when imoporting modules absolute reference. 

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


    impl Display for PersonId{
        fn fmt(&self, f: &mut Formatter<'_>) -> Result{
            // write!(f, "{} {}", self.id)
            let result = match self{
                PersonId::Passport(value) => {
                    write!(f, "The Passport Number is {}", value)
                },
                PersonId::IdentityCard(value) => {
                    write!(f, "The Identity card Number is {}", value)
                },
                // _ => Err("Error reading Id".to_owned()),
            };

            result
        }
    }

    pub struct Person{
        name:String,
        pub age: i32,
        pub profession:String,
        pub id: PersonId,
    }


    impl Log for Person {
        fn send_shout_out(&self) {
            println!("{} Says Hello", self.name);
        }


        fn display_info(&self){
            crate::outsider();

            super::outsider();

            //If in another top level module then super:;super::outsider();
            println!("{} is {} years old and is a {}, identified by {}", self.get_name(), self.age, self.profession, print_id(&self.id));
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

        pub fn get_name(&self) -> String {
            println!("From getter {}", self.name);
            return self.name.to_string()
        }

        pub fn set_name(&mut self, name: String) {
            self.name = name;
        }

        pub fn change_age(&mut self, new_age: i32){
            self.age = new_age;
        }

        pub fn set_profession(&mut self, profession: String) {
            self.profession = profession;
        }

        pub fn set_id(&mut self, id: PersonId) {
            self.id = id;
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


    pub fn log_info(value: &impl Log) {
        value.display_info();
        value.send_shout_out();
    }

    pub fn log_info2(value: &dyn Log) {
        value.display_info();
        value.send_shout_out();
        value.alert();
    }

}