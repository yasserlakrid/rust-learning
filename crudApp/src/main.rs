use std::collections::HashMap;
use std::net::{TcpListener ,  TcpStream };
use std::sync::{Arc , Mutex }; 
use serde::{Serialize, Deserialize};
use serde_json::json;
use std::io::{Read  , Write }; 
use std::thread;
use std::fmt; 
mod threadpool ; 
use threadpool::ThreadPool ;
use std::sync::atomic::{AtomicUsize , Ordering};

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
#[derive(Debug)]
enum TaskErr{
    TaskNotFound(i32),
    InvalidInput(String),
    Io(std::io::Error)
}
impl fmt::Display for TaskErr{
    fn fmt(&self , f: &mut fmt::Formatter<'_>)-> fmt::Result{
        match self {
            TaskErr::TaskNotFound(id) => write!(f , "task {} not found " , id),
            TaskErr::InvalidInput(input) => write!(f , "{} is invalid"  , input),
            TaskErr::Io(err) => write!(f, "Io err {}" , err)
         }
    }
}
impl std::error::Error for TaskErr {}
impl From<std::io::Error> for TaskErr {
    fn from(e : std::io::Error)-> Self {
        TaskErr::Io(e)
    }
}

impl Tasks {
    fn new()->Self{
        Tasks {
            tasks : HashMap::new() , next_id : 1 
        }
    }
    
    fn create(&mut self , title : String)-> Result<&Task , TaskErr> {
        let id = self.next_id ; 
        
        let new_task = Task {
            name : title , id : id , done : false 
        };

        self.next_id += 1 ; 

        self.tasks.insert(id , new_task) ; 
        self.tasks.get(&id).ok_or(TaskErr::TaskNotFound(id))
    }

    fn update(&mut self , title : Option<String> , done : Option<bool> , id : i32 ) -> Result<&Task , TaskErr> {
        let task = self.tasks.get_mut(&id).ok_or(TaskErr::TaskNotFound(id)).unwrap() ; 
        if let Some(t) = title {task.name = t} ; 
        if let Some(d) = done {task.done = d } ;
        Ok(task)
    }
    fn read(&self , id : i32)->Result<&Task , TaskErr>{
        self.tasks.get(&id).ok_or(TaskErr::TaskNotFound(id))
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
fn handle_connection(mut stream:  TcpStream , tasks : Arc<Mutex<Tasks>> ){
    
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
    let mut owned = String::from(""); 
    let mut response = String::new(); 
    {
    let mut tasks = match tasks.lock()  {Ok(tasking) => {tasking} , Err(poisen) => {return}};
    match (method,path) {
            ("GET" , "/") => {
                response = String::from("<h1>put something on the path nigga</h1>"); 
                status = "200 OK"
            },
            ("GET" , p ) => {
                if p.starts_with("/tasks/") {
                     let id = p[7..].parse().unwrap_or(-1);
                     let task = tasks.read(id); 
                     match task {
                        Ok(task) => {
                             let json = serde_json::to_string(&task).unwrap(); 
                             response = json;
                             status = "200 OK";
                        } , 
                        Err(err) => {
                            response = json!({
                                "status": err.to_string()
                            }).to_string();
                            
                            status = "400 NOT FOUND";
                        }
                     }
                }else{
                    let tasks_vec = tasks.read_all(); 
                    let json = serde_json::to_string(&tasks_vec).unwrap(); 
                    response =  json ;
                    status = "200 OK"
                }
              
            },
            ("POST" , "/tasks") => {
                let input : TaskInput = serde_json::from_str(body).unwrap();
                match tasks.create(input.title.unwrap_or("".to_string())){
                    Ok(task) =>{ response = serde_json::to_string(&task).unwrap() ;  status = "200 OK" ; }, 
                    Err(err) => {response = err_404 ; owned = format!("400 {err}"); status = &owned; } ,
                } 

            } ,
            ("PUT" , p) if p.starts_with("/tasks/") => {
                let id = p[7..].parse().unwrap_or(-1);
                let update : TaskInput = serde_json::from_str(body).unwrap();
                match tasks.update(update.title , update.done , id ) {
                    Ok(task) => {response = serde_json::to_string(task).unwrap();  status = "200 OK"  ; },
                    Err(err) =>{ 
                        response = err_404 ; 
                        owned = format!("400 {err}");
                        status = &owned;
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
fn main(){
let operations_counter = Arc::new(AtomicUsize::new(0));
let threadpool = ThreadPool::new(4  , operations_counter ); 


let tasks = Arc::new(Mutex::new(Tasks::new())); 

tasks.lock().unwrap().create(String::from("eat lunch")); 
tasks.lock().unwrap().create(String::from("eat dinner")); 

let connection = TcpListener::bind("127.0.0.1:7777"); 
let connection = match connection {
    Ok(connection )=> {
        println!("the server is running on Port : 7777"); 
        connection
    } ,
    Err(err) => {println!("Failed to bind to port 7777: {}", err); TcpListener::bind("127.0.0.1:7778").unwrap()}
};
for stream in connection.incoming() {

    println!("new connection sets"); 
    let tasks = Arc::clone(&tasks);
    let mut stream = stream.unwrap(); 
    threadpool.sendtopool( || {
        handle_connection(stream , tasks );
    }) 
}
}
