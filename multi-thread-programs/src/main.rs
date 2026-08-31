use std::net::{TcpListener , TcpStream};
use std::io::{Read , Write , BufRead , BufReader }; 
use std::thread ; 

fn main() {
    let mut threads = Vec::new(); 
    let mut x = 0; 
    for _ in 1..=5 {
        let thread = thread::spawn(move || {
            x += 1 ;
            x 
        }); 
        threads.push(thread)
    }
    let mut new_x = 0 ; 
    let mut i = 0 ; 
    for thread in threads {
            i+=1 ; 
            if i == 5{
             new_x = thread.join().unwrap(); 
            }
    }
    
    println!("if this value : {} isn't 1 than i'm stupid " , new_x); 

}   
