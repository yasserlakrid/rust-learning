
#[allow(dead_code)]
 struct Node {
       value : i32 , 
        prev : Option<Box<Node>> , 
    }
    pub struct Mstack {
         top :  Option<Box<Node>> 
    }

    
 
    impl Mstack {
        pub fn news() -> Self{
       
        Mstack {
            top : None,
        }
    }
        pub fn top (&self)->i32{
            match &self.top {
                Some(head) => head.value,
                None => -1
            }
        }
        
        pub fn push(&mut self  , value : i32){
            let sav = self.top.take();

            self.top= Some(Box::new(Node{
                value :value , prev : sav 
            }));
           

        }
        pub fn pop(&mut self)->i32{
            match self.top.take() {
                Some( head) =>  {
                   
                    let sav = head; 
                    self.top = sav.prev ; 
                    sav.value
                },
                None => {println!("the stack is empty nigga");-1 }
            }
            
        }
        
    


    }
