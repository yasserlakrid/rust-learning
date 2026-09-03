use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc , Mutex };

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool{
    pub threads : Vec<JoinHandle<()>>,
    pub sender  : mpsc::Sender<Job> 
}
impl ThreadPool{
    pub fn new(size : usize)->Self{
        let (sender , receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver)); 
        let mut threads = Vec::with_capacity(size); 
        for id in 0..size {
            
            let receiver = Arc::clone(&receiver); 
           
            let thread = thread::spawn(move || loop{
                 let job = receiver.lock().unwrap().recv(); 

                match job {
                    Ok(job) => {job(); println!("the {} is running a job", id); } ,
                    Err(err) => {println!("the server didn't receive a job to execute instead {} " , err);} 
                }
            });
            threads.push(thread);
        }
        ThreadPool {
            threads , sender
        }
    }
    pub fn sendtopool<F>(&self , job : F )
        where F : FnOnce() + Send + 'static
    {
        self.sender.send(Box::new(job)) ;
    }
}