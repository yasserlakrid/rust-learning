use std::time::Duration;
fn main(){
    trpl::block_on(async {
            trpl::spawn_task(async{
                        for i in 1..5 {
            println!("inner i = {}", i);
            trpl::sleep(Duration::from_secs(1)).await;
        }
        });
        for i in 1..5 {
        println!("outer i = {}", i);
        trpl::sleep(Duration::from_secs(1)).await;
    }
    });
    println!("main thread end");
}