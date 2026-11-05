use std::{
    collections::HashMap,
    fmt,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    thread,
};

use serde::{Deserialize, Serialize};
use serde_json::json;

mod dbconnection;
mod threadpool;

use dbconnection::DBconnection;
use threadpool::ThreadPool;
mod sqlParse;
use sqlParse::{extract_details, trait_message, Value};

#[derive(Clone , Debug, Serialize )]
pub struct Task {
    name : String , id : i32 , done : bool 
}
pub struct Tasks {
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
    
    fn create(&mut self ,db : &DBconnection , title : String)-> Result<Task , TaskErr> {
        let id = self.next_id ; 
        
        let new_task = Task {
            name : title , id : id , done : false 
        };

        self.next_id += 1 ; 
        
        let message = format!("INSERT INTO  tasks (name , Done ) VALUES ('{}', FALSE ) {}" ,new_task.name , ";"); 
        db.send(&message); 

        Ok(new_task)
    }

    fn update(&mut self , title : Option<String> , done : Option<bool> , id : i32 , db : &DBconnection) -> Result<Task , TaskErr> {
        
        let name =  match &title {
            Some(title) => title , 
            None => return Err(TaskErr::TaskNotFound(404)),
        };

        let mut task = self.read( name , db); 
        let mut task =  match task {
            Ok(task) => {
              task  
                
            },
            _ => { return Err(TaskErr::TaskNotFound(404)) ; } ,
        }; 
       
        match (title, done){
            (Some(title) , Some(done))=>{
                let message = format!("UPDATE tasks SET done = {} WHERE name = '{}' ;" , done ,  title); 
                println!("{:?}" , message);

                db.send(&message); 
                let mut start : usize = 0 ;
                let mut res = db.get();
                println!("the data base responded with : {:?}" , res); 
                task.done = done ;
            },
            (Some(title) , None)=>{
                
            },
            (None , Some(done))=>{

            },
            (None ,None )=>{

            },
        }
      

        Ok(task)
    }
    fn read(&self , name : &str , db : &DBconnection)->Result<Task , TaskErr>{
        let message = format!("SELECT * from tasks WHERE name = '{}' {}" , name , ";"); 
        println!("{:?}" , message);
        db.send(&message); 
        let mut start : usize = 0 ;
        let mut res = db.get();
        println!("the full response is {:?}" , res); 
        let mut r = 0 ; 
       
        let mut rowDescription = res.extract_message(&mut r); 

        let mut dataRow = res.extract_message(&mut r ); 
        let mut rowDescription = rowDescription.handle_description().unwrap(); 
        let mut dataRow = dataRow.handle_row(&rowDescription); 
        
       let mut dataRow =  match &dataRow {
            Some(data) => data , 
            None => return Err(TaskErr::TaskNotFound(404)) ,  
        };
        
        let mut new_task = Task {
            name : "".to_string(), id : 0 , done : false
        };
        
        for value in &dataRow.values {
        
            match value {
                Value::I32(n) => new_task.id = *n ,
                Value::String(s) => new_task.name = s.to_string() ,
                Value::Bool(s) => new_task.done = *s ,
                
                _ => println!("Another type"),
            }
        }
        Ok(new_task)


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
fn handle_connection(mut stream:  TcpStream , tasks : Arc<Mutex<Tasks>> , db : Arc<DBconnection>){
    

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
                     let name = &p[7..] ; 
                
                     let task = tasks.read(name , &db); 
                     
                     match task {
                        Ok(task) => {
                            println!("the task was found "); 
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
                match tasks.create(&db,input.title.unwrap_or("".to_string())){
                    Ok(task) =>{ response = serde_json::to_string(&task).unwrap() ;  status = "200 OK" ; }, 
                    Err(err) => {response = err_404 ; owned = format!("400 {err}"); status = &owned; } ,
                } 

            } ,
            ("PUT" , p) if p.starts_with("/tasks/") => {
               
                
                let id = p[7..].parse().unwrap_or(-1);
                println!("the id is {}" , id);
                let update : TaskInput = serde_json::from_str(body).unwrap();
                match &tasks.update(update.title , update.done , id , &db ) {
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

let connection = TcpListener::bind("127.0.0.1:7777"); 
let connection = match connection {
    Ok(connection )=> {
        println!("the server is running on Port : 7777"); 
        connection
    } ,
    Err(err) => {println!("Failed to bind to port 7777: {}", err); TcpListener::bind("127.0.0.1:7778").unwrap()}
};
let db = DBconnection::new("postgres" , "tasks" , "127.0.0.1:5432" ); 
let db  = match db {
    Some(db) => db , 
    None => db.unwrap()
};

let db = Arc::new(db); 

for stream in connection.incoming() {
    let db = Arc::clone(&db); 
    println!("new connection sets"); 
    let tasks = Arc::clone(&tasks);
    let mut stream = stream.unwrap(); 
    threadpool.sendtopool( || {
        handle_connection( stream , tasks , db );
    }) 
}
}
