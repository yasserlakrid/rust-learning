use std::net::{TcpListener ,  TcpStream };
use std::sync::mpsc;
use std::sync::{Arc, Mutex} ; 

use std::thread ; 
pub struct DBconnection {
    sender : mpsc::Sender<String> ,
    connection : Arc<Mutex<TcpStream>>
}

impl DBconnection {
    pub fn new(port : &str)-> Option<Self> {
        let connection = TcpStream::connect(port); 
        let connection = match connection {
            Ok(connection) => connection , 
            Err(err) => return  None
        }; 
        let (sender , receiver) = mpsc::channel(); 
        let connection = Arc::new(Mutex::new(connection)); 
        let connection = Arc::clone(&connection); 
       thread::spawn(move || loop{
            let query = receiver.recv(); 

            println!("new request arrived to be send to the data base {}" , query.unwrap()); 
       });
        let connection = Arc::clone(&connection); 
        Some(DBconnection {sender , connection})
    }
    pub fn send(&self , query : &str ) {
        let sender = &self.sender ;
            sender.send(query.to_string()).expect("thread pool workers have stopped");
    }
    
}