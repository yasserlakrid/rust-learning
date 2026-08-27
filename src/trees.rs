
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
    pub fn search(&self , value : i32) -> bool {
        match &self.root {
            Some(root)=> {
                root.search(value)
            },
            None => false 
        }
    }
    pub fn delete(&mut self , target : i32)->bool{ 
        if let Some(root) = &self.root {
            
            return delete(&mut self.root , target)
        }
        return false
    }

}
fn delete(root : &mut Option<Box<Node>> , target : i32 )->bool{
    return match root {
        Some(node)=>{//node is Node
            if node.value < target {
                return delete(&mut node.rc , target)
            }else if node.value > target {
                return delete(&mut node.lc , target)
            }
            match (&node.rc , &node.lc) {
                (None , None ) => {*root = None}
                (Some(r) , None) => {*root = node.rc.take()}
                (None , Some(l)) => {*root =  node.lc.take()}
                (Some(rc) , Some(lc)) => {
                    let mut succ = rc ; //succ is &box node 
                    while let Some(next) = succ.lc.as_ref() { // next is &box node
                        succ = next ;
                    };
                    node.value = succ.value ; 
                    delete(&mut node.rc , node.value) ; 
                    
                }
            }   
            true
        },
        None=>false
    }
}

impl Node {
    
    fn print(&self){
            if let Some(left_child) = &self.lc {
                left_child.print();

            }

            println!("{}" ,  self.value);
            
            if let Some(right_child) = &self.rc {
                right_child.print();

              }
    }
     fn my_clone(&self) -> Node {
        Node {
            value: self.value,
            rc: match &self.rc {
                Some(right) => Some(Box::new(right.my_clone())) ,
                None => None
            } ,  
            lc: match &self.lc {
                Some(left) => Some(Box::new(left.my_clone())) ,
                None => None
            } 
        }
    }
     fn insert(&mut self , value_in : i32){
      
           
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
    fn search(&self , value : i32) -> bool {
       
            
                if self.value == value {
                    true
                }else{
                    if self.value > value {
                        match &self.lc {
                            Some(left_child)=>{
                             left_child.search(value)
                            },
                            None => false
                        }
                    }else{
                         match &self.rc {
                            Some(right_child)=>{
                             right_child.search(value)
                            },
                            None => false
                        }
                       
                    }
                }
            
            
            }

        }