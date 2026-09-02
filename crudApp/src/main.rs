use std::collections::HashMap;
use std::net::{TcpListener ,  TcpStream };
use std::sync::{Arc , Mutex }; 
use serde::{Serialize, Deserialize};
use std::io::{Read  , Write }; 
use std::thread;
mod threadpool; 
use threadpool::ThreadPool ; 

#[derive(Clone , Debug, Serialize )]
struct Task {
    name : String , id : i32 , done : bool 
}
struct Tasks {
    tasks : HashMap<i32  , Task > , next_id : i32 
}

#[derive(Deserialize)]
struct TaskInput{
    title : Option<String>,
    done : Option<bool> 
}

impl Tasks {
    fn new()->Self{
        Tasks {
            tasks : HashMap::new() , next_id : 1 
        }
    }
    
    fn create(&mut self , title : String)-> Option<&Task> {
        let id = self.next_id ; 
        
        let new_task = Task {
            name : title , id : id , done : false 
        };

        self.next_id += 1 ; 

        self.tasks.insert(id , new_task) ; 
        self.tasks.get(&id)
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
fn handle_connection(mut stream:  TcpStream , tasks : Arc<Mutex<Tasks>>){
    println!("creating a new thread only for your connection you're lucky dumbass");
    
    let mut buf = [0;1024] ; 
    let n = stream.read(&mut buf ).unwrap() ; 
    let request = String::from_utf8_lossy(&buf[..n]);
    
    let request_line = request.split("\r\n\r\n").nth(0).unwrap();
    let body  = request.split("\r\n\r\n").nth(1).unwrap();

    let mut parts = request_line.split_whitespace(); 
    let method = parts.next().unwrap_or(""); 
    let  path = parts.next().unwrap_or(""); 


    let err_404 = String::from(r#"{"status": 404, "code": "no task available with this id"}"#) ; 
    let mut status  : &str = ""; 

    let mut response = String::new(); 
    {
    let mut tasks = match tasks.lock()  {Ok(tasking) => {tasking} , Err(poisen) => {return}};
    match (method,path) {
            ("GET" , "/") => {
                response = String::from("<h1>put something on the path nigga</h1>"); 
                status = "200 OK"
            },
            ("GET" , "/tasks") => {
                let tasks_vec = tasks.read_all(); 
                        let json = serde_json::to_string(&tasks_vec).unwrap(); 
                        response =  json ;
                        status = "200 OK"
            },
            ("POST" , "/tasks") => {
                let input : TaskInput = serde_json::from_str(body).unwrap();
                match tasks.create(input.title.unwrap_or("".to_string())){
                    Some(task) =>{ response = serde_json::to_string(&task).unwrap() ;  status = "200 OK" ; }, 
                    None => {response = err_404 ; status = "400 NOT FOUND"; } ,
                } 

            } ,
            ("PUT" , p) if p.starts_with("/tasks/") => {
                let id = p[7..].parse().unwrap_or(-1);
                let update : TaskInput = serde_json::from_str(body).unwrap();
                match tasks.update(update.title , update.done , id ) {
                    Some(task) => {response = serde_json::to_string(task).unwrap();  status = "200 OK"  ; },
                    None =>{ 
                        response = err_404 ; 
                        status = "400 NOT FOUND";
                    } ,
                }
                
            },
            ("DELETE" , p) if p.starts_with("/tasks/") => {
                let id = p[7..].parse().unwrap_or(-1);
                match tasks.delete(id) {
                    Some(task) => {
                        response = serde_json::to_string(&task).unwrap() ;
                        println!("{:?} is deleted successefuly" , task);
                        status = "200 OK" ; 
                    },
                    None => {
                        response = err_404 ;
                        status = "400 NOT FOUND" ; 
                    }
                }
            }
            _=> { 
                response = String::from(r#"{"status": 404}"#); 
            }

        }
        }
    println!("{} , {}" , method , path);
   
    let http_response = format!(
        "HTTP/1.1 {}\r\nContent-type: application/json\r\n\r\n{}" , status ,response
    );
    stream.write_all(http_response.as_bytes()); 
}
fn main() {
    let threads = ThreadPool::new(4);
    let tasks = Arc::new(Mutex::new(Tasks::new())); 
tasks.lock().unwrap().create(String::from("eat lunch")); 
tasks.lock().unwrap().create(String::from("eat dinner")); 

let connection = TcpListener::bind("127.0.0.1:7777").unwrap(); 
for stream in connection.incoming() {

    println!("new connection sets"); 
    let tasks = Arc::clone(&tasks);
    let mut stream = stream.unwrap() ; 
       
    threads.sendthistopool( move || {handle_connection(stream , tasks)});
}
}
