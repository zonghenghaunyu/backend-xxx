pub mod al;
pub mod classification;
pub mod db;
pub mod demo;
pub mod fifth;
pub mod first;
pub mod forthtwo;
pub mod second;
pub mod third;
// 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

use std::time::{Duration, SystemTime};
fn hello(){
    std::thread::sleep(Duration::from_millis(1000));
    let time = SystemTime::now();

    println!("{:?}",time)
}

async fn hello1(){

    std::thread::sleep(Duration::from_millis(1000));
    let time = SystemTime::now();

    println!("{:?}",time)
}

#[cfg(test)]
mod tests {
    use crate::{hello, hello1};

    #[test]
    fn it_works() {
        // hello();
        // hello();
        // hello();
        // hello();
        hello1().await;
        hello1().await;
        hello1().await;
        hello1().await;
    }



}
