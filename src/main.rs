#[allow(dead_code)]
mod my_stack ; 
use my_stack::Mstack; 
mod trees ; 
use trees::Mtree;

fn main() {
    let  stack = Mstack::news() ; 
    println!("{}" , stack.top());
    let mut tree = Mtree::new(); 
    tree.insert_rc(1); 
    tree.insert_rc(2);
    tree.insert_lc(3); 


    

}
