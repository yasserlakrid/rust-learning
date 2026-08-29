use std::collections::HashMap;
use std::net::TcpListener;
use std::io::{Read  , Write }; 
#[derive(Clone , Debug)]

struct Task {
    name : String , id : i32 , done : bool 
}
struct Tasks {
    tasks : HashMap<i32  , Task > , next_id : i32 
}
impl Tasks {
    fn new()->Self{
        Tasks {
            tasks : HashMap::new() , next_id : 1 
        }
    }
    
    fn create(&mut self , title : String)-> &Task {
        let id = self.next_id ; 
        
        let new_task = Task {
            name : title , id : id , done : false 
        };

        self.next_id += 1 ; 

        self.tasks.insert(id , new_task) ; 
        self.tasks.get(&id).unwrap()
    }

    fn update(&mut self , title : Option<String> , done : Option<bool> , id : i32 ) -> Option<&Task> {
        let task = self.tasks.get_mut(&id)? ; 
        if let Some(t) = title {task.name = t} ; 
        if let Some(d) = done {task.done = d } ;
        Some(task)
    }
    fn read(&self , id : i32)->Option<&Task>{
        self.tasks.get(&id)
    }

    fn read_all(&self ) -> Vec<&Task> {
        self.tasks.values().collect()
    }

    fn delete(&mut self , id : i32)->Option<Task>{
        self.tasks.remove(&id)
    }
    fn print_tasks(&self){
        println!("the tasks are : ");
        for task in self.read_all() {
        println!("{:?}" , task );

    }
    }
}
fn main() {
   
let connection = TcpListener::bind("127.0.0.1:7777").unwrap(); 
for stream in connection.incoming() {
    println!("new connection sets"); 
    let mut stream = stream.unwrap() ; 
    let mut buf = [0;1024] ; 
    let n = stream.read(&mut buf ).unwrap() ; 
    let request = String::from_utf8_lossy(&buf[..n]);
    let request_line = request.lines().next().unwrap(); 
    let mut parts = request_line.split_whitespace(); 
    let method = parts.next().unwrap_or(""); 
    let  path = parts.next().unwrap_or(""); 
    let mut response = String::new(); 
    println!("{} , {}" , method , path); 
    match method {
        "GET" => {
            match path {
                "/tasks" => {
                    response = String::from("the tasks are : ");
                },
                _=> {response = String::from("you didn't asked about tasks nigga") ; }
            }
        }, 
        _=> { 
            response = String::from("what the fuck you put in the method nigga"); 
        }

    }
    stream.write_all(response.as_bytes()); 
}

}
