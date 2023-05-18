
use rust_basics::rust_types::*;



fn main() {
    let mut person = Person::new();
    
    person.set_name("Rodrygo Goes".to_owned());
    person.change_age(21);
    person.set_profession("Left Wing for Real Madrid CF".to_owned());
    person.set_id(PersonId::Passport("XHG678092".to_owned()));
    let person_id = &person.id;

    // format!("{person_id}");

    println!("{}", &person_id);

    person.display_info();
}