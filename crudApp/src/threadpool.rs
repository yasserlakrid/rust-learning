use std::sync::mpsc;
use std::thread;
use std::sync::{Arc , Mutex };

type Job = Box<dyn FnOnce() + Send +'static>;

pub struct ThreadPool{
    pub pool : Vec<thread::JoinHandle<()>> ,
    pub sender : mpsc::Sender<Job>
}
impl ThreadPool {
    pub fn new(size : usize)->Self{
        let (sender , receiver) = mpsc::channel::<Job>(); 
       let receiver = Arc::new(Mutex::new(receiver));
        let mut handlers =  Vec::with_capacity(size); 
        for id in 0..=size{
            let receiver = Arc::clone(&receiver);

            let thread = thread::spawn(move || loop{
                let job = receiver.lock().unwrap().recv(); 
                match job {
                    Ok(job) => {job(); println!("thread {} is executing a job" , id);},
                    Err(err) => {break ;}
                }

                
            });
            handlers.push(thread); 
        }
        ThreadPool {pool : handlers , sender}
    }
    pub fn sendthistopool<F>(&self , job : F )
    where F :  FnOnce() + Send +'static {
        self.sender.send(Box::new(job)).unwrap()
    }
}