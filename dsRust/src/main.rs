#[allow(dead_code)]

mod trees ; 
use trees::Mtree;

fn main() {
    
    let mut tree = Mtree::new();
     tree.insert(4);
    tree.insert(1);
    
    tree.insert(10);
    tree.insert(2);

    tree.delete(2); 
    tree.print(); 
    
    println!("{:?} {:?}"  , tree.search(4) , tree.search(2) );

    

}
