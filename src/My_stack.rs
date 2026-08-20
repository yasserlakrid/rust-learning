
    struct Node {
         value : i32 , 
         prev : Option<*mut Node> , 
    }
    pub struct Mstack {
        pub top :  Option<*mut Node>  
    }
    pub fn News(head : i32) -> Mstack{
        let sav = Node {
            value : head , 
            prev : None , 
        };
        Mstack {
            top : sav ,
        }
    }
    impl Mstack {
        pub fn Push(&mut self  , value : i32){
            let  sav = &mut self.top; 
            sav.value = value ; 
            self.top.prev =Some( &mut *sav );
        }
        
        pub fn Pop(mut self )->i32{
            
            let sav = &mut self.top ; 
            if let Some(previous) = &sav.prev {
                    self.top = *previous ; 
            }else{
                println!("the stack is empty "); 

            };
            sav.value
            
            
            
        }


    }
