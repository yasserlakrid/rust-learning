use std::net::{TcpListener , TcpStream};
use std::sync::{Mutex , Arc}; 
use std::io::{Read , Write , BufRead , BufReader }; 
use std::thread ; 

fn main() {

    let data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let handle = std::thread::spawn(move || {
        let data = Arc::clone(&data); 
        
        println!("{:?}", data.lock().unwrap());
    });

    handle.join().unwrap();

}   
