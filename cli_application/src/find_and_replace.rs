use text_colorizer::*;
#[allow(unused_imports)]
use rand::*;
use std::env;
use std::io::stdin;
use std::fs;
use regex::Regex;



#[derive(Debug)]
#[allow(dead_code)] //suppress warnings
struct Arguments{
    pattern: String,
    replace:String,
    input_file:String,
    output_file:String
}


fn print_help(){
    eprintln!("{} - replace a string with a new string", "Find and Replace".green()); //output goes to standard error except standard out, used for errors and progress messages. Change color like .green
    eprintln!("{}: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>", "Usage".blue());
}

fn parse_args() -> Arguments{
    let args: Vec<String> = env::args().skip(1).collect(); //skip 1 bercause upon running, that is the first argument
    if args.len() != 4{
        print_help();
        eprintln!("{} - Incorrect number of arguments, expected 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1); //err code 1 signifies it did not run successfully
    } else {
        for i in &args{
            eprintln!("You Entered {}, Is this correct", &i.yellow().bold());
        }

        let mut res = String::new();
        println!("If args are correct enter Yes else No");
        let confirmation = stdin().read_line(&mut res).unwrap();
        println!("User Response is {} ", res.green().bold());

        if res.trim_end().eq("Yes"){
            println!("Got it");
            Arguments{pattern: args[0].clone(), replace: args[1].clone(), input_file:args[2].clone(), output_file:args[3].clone()}
        } else {
            eprintln!("{} Incorrect input entered, please select your choice Yes or No", "Error".red().bold());
            parse_args();
        }

    }


}

fn read_and_write(args: &Arguments){
    let data = match fs::read_to_string(&args.input_file){
        Ok(f) => f,
        Err(e) => {
            println!("{} Failed to read from file {}: {:?}", "Error".red().bold(), args.input_file, e);
            std::process::exit(1);
        }
    };

    let replace_data = match replace(target: args.pattern, replace: arge.replace, &data){
        Ok(d) => d,
        Err(e) => {
            println!("{} Failed to replace text, {:?}", "Error".red().bold(),  e);
            std::process::exit(1);
        }
    }

    match fs::write(&args.output_file, &replace_data){
        Ok(_) => {},
        Err(e) => {
            println!("{} Failed to write to file {}: {:?}", "Error".red().bold(), args.output_file, e);
            std::process::exit(1);
        }
    }
}


//data is stuff from file
fn replace(target: &str, replace: &str, data: &str) -> Result<String, regex::Error>{
    let regex = Regex::new(target)?;
    Ok(regex.replace_all(data, replace).to_string()) 
}

pub fn run() {
    let args = parse_args();
    println!("{:?}", args);
    read_and_write(&args);
}