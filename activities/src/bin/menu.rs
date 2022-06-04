#[derive(Debug)]
enum MenuChoice{
    MainMenu,
    Start,
    Quit,
}

//function returns an ok or an err.
//menu chocie wrapped in an ok

fn get_choice(user_input: &str) -> Result<MenuChoice, String>{
    match user_input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("You Entered Inappropriate Detal, Please choose from the list of options".to_owned()),
    }
}


fn print_choice(choice: &MenuChoice){
    println!("choice = {:?}", choice);
}

//inner data is a menu choice here as it is wrapped in a result. 
// We need to access this inner value 
//We match on the chocie to get the inner data werapped and use the print choice to get the inner data. 
// We can use the ? wildcard operator to match on anything available.
//unit type () returns nothing. This way we can return nothing if we need. 
//? automatically performs a match operation. if result is OK, the inner data gets placed in choice, else error is returned. 

fn pick_choice(input: &str) ->Result<(), String> {
    let choice  = get_choice(input)?;
    print_choice(&choice);
    Ok(())

}

fn main() {
    let choice: Result<MenuChoice,_> = get_choice("leave");
    // println!("choice = {:?}", choice);
    // print_choice(&choice);
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e),
    }


    println!("___________________________________________________");
    pick_choice("start");

}