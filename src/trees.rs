
#[allow(dead_code)]

struct Node {
    value : i32 , lc : Option<Box<Node>> , rc : Option<Box<Node>> 
}
pub struct Mtree {
    root : Option<Box<Node>>
}
impl Mtree {
 

    pub fn new()->Self{
        Mtree {
            root : None
        }
    }

    pub fn insert(&mut self  , value_in : i32 ){
        match &mut self.root {
            Some(root)=>{
                root.insert(value_in);
            },
            None => {
                self.root = Some(Box::new(Node {
                    rc : None , lc: None , value : value_in
                }))
            }
        }
    }
    
    pub fn print(&self){
        if let Some(root) = &self.root  {
            root.print()
        }
       
    }
}
impl Node {
    pub fn print(&self){
            if let Some(left_child) = &self.lc {
                left_child.print();

            }

            println!("{}" ,  self.value);
            
            if let Some(right_child) = &self.rc {
                right_child.print();

              }
    }

    pub fn insert(&mut self , value_in : i32){
      
           
                if self.value < value_in {
                    match &mut self.rc {
                        Some(node) => node.insert(value_in),
                        None => self.rc = Some(Box::new(Node {
                            rc : None, lc : None , value : value_in
                        }))
                    }
                }else{
                    match &mut self.lc {
                        Some(node) => node.insert(value_in),
                        None => self.lc = Some(Box::new(Node {
                            rc : None, lc : None , value : value_in
                        }))
                    }
                }
        

           
        }
    } 
