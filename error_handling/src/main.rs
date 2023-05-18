use std::fs::File;
use std::io::ErrorKind;
use std::fs::rename;
use std::io::Error;


fn main() {
    // panic!("Panic Here");

    let mut numbers: Vec<i32> = Vec::new();
    numbers.push(1);

    let test  = open_file();
    test.unwrap();

    rename_file().unwrap();

    // println!("{:?}", numbers[10]);

    // let file = File::open("error.txt");
    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) =>  match error.kind(){
    //         ErrorKind::NotFound => match File::create("Error.text"){
    //             Ok(file_created) => file_created,
    //             Err(error) => panic!("Cannot create the file"),
    //         }
    //         _ => panic!("It was another error Kind"),
    //     }
    // };

    // let file_two = File::open("Error.txt").unwrap();
    // let file_two = File::open("Error.txt").expect("File Not Found, please check source name.");
}

fn open_file() -> Result<File, Error>{
    let file = File::open("Error.txt")?; //uses ? to propagate error up
    Ok(file)
}

fn rename_file() -> Result<(), Error>{
    let file = rename("Error.txt", "Error2.txt")?;
    Ok(file)
}

// enum Result<T,E>{
//     Ok(T),
//     Err(E)
// }