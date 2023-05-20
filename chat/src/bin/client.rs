use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::{task, io, net};
use std::sync::Arc;

use chat::utils::(self, ChatResult);
use chat::{Client, Server};


//get value gets first value forom input string and separate by white space. input.

fn get_value(mut input: &str) -> Option<(&str, &str)>{
    input = input.trim_start(); //removes white space that is leading. 

    if input.is_empty(){
        return None;
    }

//if whitespace is found, split input into tuple and return what we need/
    match input.find(char::is_whitespace){
        Some(whitespace) => Some((&input[0...whitespace], &input[whitespace..])),
        None => Some((&input, "")),
    }
}

fn parse_input(line: &str) -> Option<Client>{
    let (input, remainder) = get_value(line)?;

    if input == "join"{
        let (chat, remainder) = get_value(remainder)?;

        if !remainder.trim_start().is_empty(){
            return None;
        }

        return Some(Client::Join{chat_name: Arc::new(chat.to_string()),});
    }

    else if  input == "post"{
        let (chat, remainder) = get_value(remainder);

        let message = remainder.trim_start().to_string();

        return Some(Client::Post{chat_name: Arc::new(chat.to_string()), message: Arc::new(message)});
    } else {
        println!("Unrecognized input: {:?}", line);
        return None;
    }
    
}

//Function to send messages to server
async fn send(&mut send: net::TcpStream) -> ChatResult<()>{
    println!("Options: \njoin CHAT \npost CHAT MESSAGE"); //What we expect from users.

    let mut options = io::BufReader::new(io::stdin()).lines(); //buffered reader for cmd line

    while let Some(option_result) = options.next().await{
        let opt = option_result?;
        let req = match parse_input(&opt){
            Some(req) => req, 
            None => continue,
        };

        utils::send_json(&mut send, &req).await?;
        send.flush().await?; //flush ensures all data is sent
    }
    Ok(())

}

async fn messagess(server: net::TcpStream) -> ChatResult<()>{
    let buf = io::BufReader::new(server);

    let mut stream = utils::recieve(buf); //creates a new stream to recieve messages from server

//go through all messages from server
    while let Some(msg) = stream.next().await{
        match msg? {
            Server::Message {chat_name, message} => {
                println!("Chat Name: {}\n, Message: {}\n", chat_name, message);
            }
            Server::Err(message) => {
                printline("Error Received: {}, message")
            }
        }
    }
    Ok(())
}

fn main() -> ChatResult<()>{
    let addr = std::env::args().nth(1).expect("Address:PORT"); //Read in server address and port from cmd.

    task::block_on(async {
        let socket = net::TcpStream::connect(addr).await?; //connect to server using addr and port
        socket.set_nodelay(true); //enable algo to reduce latency, segments are sent as soon as possible, when not set data is buffered. Send data as soon as psossible
        let send = send(socket.clone()); //create a new task to send messages to server
        let replies = messages(socket); //task to recieve messaged from server

        replies.race(send).await?; //allows the two task to run concur and wait for one to complete either send or reply.
        Ok(())
    })

}