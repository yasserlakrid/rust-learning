#[allow(dead_code)]
mod my_stack ; 
use my_stack::Mstack; 
mod trees ; 
use trees::Mtree;

fn main() {
    
    let mut tree = Mtree::new(); 
    tree.insert(1);
    tree.insert(4);
    tree.insert(10);
    tree.insert(2);
    tree.print(); 


    

}
