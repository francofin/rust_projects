// Project 1: Interactive bill manager
//
// User stories:
// * L1: I want to add bills, including the name and amount owed.
// * L1: I want to view existing bills.
// * L2: I want to remove bills.
// * L3: I want to edit existing bills.
// * L3: I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at level 1, but a
//   hashmap will be easier to work with at levels 2 and 3.
// * Create a function just for retrieving user input, and reuse it
//   throughout your program.
// * Create your program starting at level 1. Once finished, advance to the
//   next level.
use std::io;
use std::collections::HashMap;


#[derive(Debug, Clone)]
struct Bill{
    name:String,
    amount:f64,
}

//Inner field means we take the inner value of the bills. 
//This bills strcutre allows us to manage code much easier. 


#[derive(Debug, Clone)]
struct Bills{
    inner: HashMap<String, Bill>,
}

impl Bills {
    fn new() -> Self{
        Self {inner: HashMap::new()}
    }


    fn add_bills(&mut self, bill: Bill) {
        //When working with our bills we need to make sure that we have a mutable version of the bills.
        //We are mutating it when we add a field 
        self.inner.insert(bill.name.clone(), bill);
    }


    //view all transfers wonership to who ever calls the fucntion. 

    fn view_all_bills(&self) -> Vec<Bill> {

        let mut bills = vec![];
        for bill in self.inner.values(){
            //.values always returns a refernces.
            bills.push(bill.clone());
        }

        bills
        
    }

    fn remove_bill(&mut self, name: &str) -> bool {
        self.inner.remove(name).is_some()
        //removing from the hashmap returns an option. thus we can get a some or a none value. 
    }

    fn update_bill(&mut self, name: &str, amount: f64) -> bool {
        match self.inner.get_mut(name){
            //get optional mutable value, if not found its none. 
            Some(bill) => {
                bill.amount =amount;
                true
            },
            None => false,
        }
    }

}

fn get_bill_amount() -> Option<f64> {
    println!("Amount");
    loop{
        let input = match get_input() {
            Some(input) => input,
            None => return None,
        };
        if &input == "" {
            return None;
        }
        //Parse String
        let parsed_input: Result<f64, _> = input.parse();
        match parsed_input {
            Ok(amount) => return Some(amount),
            Err(_) => println!("Please Enter a valid number"),
        }

    }
}


fn remove_bill_menu(bills: &mut Bills){
    for bill in bills.view_all_bills(){
        println!("{:?}", bill);
    }
    println!("Enter the product name");
    let input = match get_input() {
        Some(input) => input,
        None => return,
    };
    if bills.remove_bill(&input){
        println!("Removed")
    } else {
        println!("Bill not found")
    }

}

fn update_bill_menu(bills: &mut Bills){
    for bill in bills.view_all_bills(){
        println!("{:?}", bill);
    }
    println!("Update the product name");
    let name = match get_input() {
        Some(name) => name,
        None => return,
    };
    let amount = match get_bill_amount(){
        Some(amount) => amount, 
        None => return,
    };
    if bills.update_bill(&name, amount) {
        println!("updated")
    } else {
        println!("Bill Not Found")
    }


}

fn add_bill_menu(bills: &mut Bills){
    // get bill name
    //get bill amount
    println!("Enter the product name");
    let name = match get_input() {
        Some(input) => input,
        None => return,
    };
    let amount = match get_bill_amount(){
        Some(amount) => amount, 
        None => return,
    };

    let bill = Bill {
        name,
        amount
    };

    bills.add_bills(bill);
    println!("Bill Successfully added");
    
}


fn get_input() -> Option<String>{
    let mut buffer = String::new();
    // let user_input = io::stdin().read_line(&mut buffer);
    //readline returns a result.
    while io::stdin().read_line(&mut buffer).is_err() {
        println!("Please Enter Your input data again")
    }
    //trim returns a borrowed string. 
    let input  = buffer.trim().to_owned();

    if &input == ""{
        None
    } else {
        Some(input)
    }
    
    // match &input{
    //     value => return Some(value.as_str()),
    //     "" => None,
    // }
}


fn view_bills_menu(bills: &Bills){
    for bill in bills.view_all_bills() {
                println!("{:?}", bill)
            }
}



fn main_menu() {

    fn show() {
        println!("");
        println!("=== Manage Bills ===");
        println!("1. Add A Bill");
        println!("2. View A Bill");
        println!("3. Remove A Bill");
        println!("4. Adjust A Bill");
        println!("Quit");
        println!("");
        println!("Enter Selection");


    }

    fn approve_quit() -> bool {
        println!("");
        println!("Are you sure you want to exit Y/N");

        let user_option = match get_input(){
            Some(user_input) => user_input,
            None => return false,
        };

        if user_option == "Y".to_lowercase() {
            true
        } else {
            false
        }
    } 

    let mut bills = Bills::new();

    loop {
        show();
        let input = match get_input(){
            Some(input) => input,
            None => return,
        };
        match input.as_str() {
            "1" => add_bill_menu(&mut bills),
            "2" => view_bills_menu(&bills),
            "3" => remove_bill_menu(&mut bills),
            "4" => update_bill_menu(&mut bills),
            "Quit" => {
                let user_option = approve_quit();
                if user_option {
                    break;
                };
            },
            _ => break,
        }
    }


}
fn main() {
    loop{
        main_menu();

        break;
    }
}
