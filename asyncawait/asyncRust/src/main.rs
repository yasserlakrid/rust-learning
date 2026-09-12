use std::time::Duration;
use trpl ;
use trpl::Either;
use std::sync::{Arc, Mutex};
async fn  slow(duration : Duration) ->Duration{
            trpl::sleep(duration).await;
            duration
        }
        async fn  timeOut<F:Future>(fut : F, duration : Duration) -> Result<F::Output, Duration>{
            match trpl::select(fut , trpl::sleep(duration)).await{
                Either::Left(output) => Ok(output),
                Either::Right(_) => Err(duration)
            }
        }

fn main(){
    trpl::block_on(async {
       
        

        let result = timeOut(slow(Duration::from_secs(4)), Duration::from_secs(2)).await;
        match result {
            Ok(output) => println!("output is {}", output.as_secs()),
            Err(duration) => println!("timed out after {} seconds", duration.as_secs())
        }
    });
   
}