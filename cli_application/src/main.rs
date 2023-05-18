use text_colorizer::*;
#[allow(unused_imports)]
use rand::*;
use std::env;

#[derive(Debug)]
#[allow(dead_code)] //suppress warnings
struct Arguments{
    pattern: String,
    replace:String,
    input_file:String,
    output_file:String
}



fn main() {
    //String to search, replace, input and output
    
    let args: Vec<String> = env::args().skip(1).collect(); //skip 1 bercause upon running, that is the first argument
    if args.len() != 4{
        print_help();
        eprintln!("{} - Incorrect number of arguments, expected 4, got {}", "Error".red().bold(), args.len());
        std::process::exit(1); //err code 1 signifies it did not run successfully
    } else {
        for i in &args{
            eprintln!("You Entered {}, Is this correct", &i.yellow().bold());
        }
    }

}


fn print_help(){
    eprintln!("{} - replace a string with a new string", "Find and Replace".green()); //output goes to standard error except standard out, used for errors and progress messages. Change color like .green
    eprintln!("{}: <target string> <replacement string> <INPUT FILE> <OUTPUT FILE>", "Usage".blue());
}