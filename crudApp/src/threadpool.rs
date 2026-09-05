use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::sync::{Arc , Mutex , atomic::{AtomicUsize , Ordering} };
type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool{
    pub threads : Vec<JoinHandle<()>>,
    pub sender  : Option<mpsc::Sender<Job>>
}
impl ThreadPool{
    pub fn new(size : usize , operations_counter : Arc<AtomicUsize> )->Self{
        let (sender , receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver)); 
        let mut threads = Vec::with_capacity(size); 
        for id in 0..size {
            
            let receiver = Arc::clone(&receiver); 
           let operations_counter = Arc::clone(&operations_counter);
            let thread = thread::spawn(move || loop{
                 let job = receiver.lock().unwrap().recv(); 
                operations_counter.fetch_add(1,Ordering::SeqCst) ;
                match job {
                    Ok(job) => {job(); println!("the thread {} is running a job number {}", id , operations_counter.load(Ordering::SeqCst)); } ,
                    Err(err) => {println!("the server didn't receive a job to execute instead {} " , err);break; } 
                }
            });
            threads.push(thread);
        }
        ThreadPool {
            threads , sender: Some(sender)
        }
    }
    pub fn sendtopool<F>(&self , job : F )
        where F : FnOnce() + Send + 'static
    {
        if let Some(sender) = &self.sender {
            sender.send(Box::new(job)).expect("thread pool workers have stopped");
        }
    }

}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        self.sender.take();

        for thread in self.threads.drain(..) {
            thread.join().expect("worker thread panicked");
        }
    }
}