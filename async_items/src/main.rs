use async_std::{fs::File, io, prelude::*, task};


async fn read_file(path: &str) ->io::Result<String>{
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}
fn main() {
    println!("Hello, world!");

    let task = task::spawn(async {
        let result = read_file("read.txt").await;

        match result {
            Ok(f) => println!("{}", f),
            Err(e) => println!("Error reading from file: {}", e),
        }
    });

    println!("Task has started");
    task::block_on(task);
    println!("Task has Ended");
}


// trait Fututre{
//     type Output;
//     fn pull(self: Pin<&mut self>, cx: &mut Context) -> Pull<Self::Output>; //cx is context.
//     //pull returns pull ready when computation is done, pull pending when not done,
//     //allows us to use the await key wqord. 
// }