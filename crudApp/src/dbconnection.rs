use std::net::{TcpListener ,  TcpStream };
use std::sync::mpsc;
use std::sync::{Arc, Mutex} ; 
use std::io::{Read , Write}; 
use std::collections::VecDeque;

use std::thread ; 
pub struct DBconnection {
    sender : mpsc::Sender<String> ,
    receiver : Arc<Mutex<mpsc::Receiver<Vec<u8>>>> ,
    connection : Arc<Mutex<TcpStream>>
}
fn buil_start_up(user : &str , db :&str) -> Vec<u8>{
   let mut message = Vec::new();

    message.extend_from_slice(&196608i32.to_be_bytes());

    message.extend_from_slice(b"user\0");
    message.extend_from_slice(user.as_bytes());
    message.push(0);

    message.extend_from_slice(b"database\0");
    message.extend_from_slice(db.as_bytes());
    message.push(0);

    message.push(0); // final parameter-list terminator

    let length = 4 + message.len();

    let mut request = Vec::new();

    request.extend_from_slice(&(length as i32).to_be_bytes());
    request.extend_from_slice(&message);

    request
}   

fn build_query(query : &str) -> Vec<u8> {
    let length = 4 + query.len() + 1 ; 
    let mut message = Vec::new(); 
    message.push(b'Q'); 
    message.extend_from_slice(&(length as i32).to_be_bytes()); 
    message.extend_from_slice(query.as_bytes());
    message.push(0);
    message
}
impl DBconnection {
    pub fn new(user : &str , db : &str,port : &str)-> Option<Self> {
        let connection = TcpStream::connect(port); 
        

        let mut connection = match connection {
            Ok(connection) => connection , 
            Err(err) => return  None
        }; 
        let startupmessage = buil_start_up(user , db);
        connection.write_all(&startupmessage);
        let mut buf = [0;1024]; 
        let n =  connection.read(&mut buf).unwrap(); 
    
       
        let (req_sender , req_receiver) = mpsc::channel::<String>(); 
        let (response_send , response_rec) = mpsc::channel::<Vec<u8>>();

        let connection = Arc::new(Mutex::new(connection));
        let response_rec = Arc::new(Mutex::new(response_rec)) ;
        let connectionArc = Arc::clone(&connection); 
       thread::spawn(move || loop{
            let query = {
                let query = req_receiver.recv().unwrap(); 
                println!("sending the request {}" , query); 
                build_query(&query)
            };
            let mut connection = connectionArc.lock().unwrap();

            connection.write_all(&query);

            let mut buf = [0;1024]; 
            let n =  connection.read(&mut buf).unwrap(); 
         
            let response =&buf[..n];
           
            response_send.send((*response).to_vec()).expect("faild response sending "); 


            
       });
        let connection = Arc::clone(&connection); 
        let response_rec = Arc::clone(&response_rec);
        Some(DBconnection {sender : req_sender , receiver : response_rec, connection })
    }
    pub fn send(&self , query : &str ) {
        let sender = &self.sender ;
            sender.send(query.to_string()).expect("thread pool workers have stopped");
    }
    pub fn get(&self) -> Vec<u8> {
        self.receiver.lock().unwrap().recv().unwrap()
    }
   
}