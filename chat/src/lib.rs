use serde::{Deserialize, Serialize};
use std::sync::Arc;
pub mod utils;

#[derive(Debug, DeSerialize, Serialize, PartialEq)] //PartialEq, used for compariosn between types. 
pub enum Client{
    Join{chat_name:Arc<String>},
    Post{chat_name: Arc<String>, message: Arc<String>, }
}

#[derive(Debug, DeSerialize, Serialize, PartialEq)]
pub enum Server {
    Message{chat_name: Arc<String>. message: Arc<String>},
    Error(String)
}