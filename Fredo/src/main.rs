mod server;
use std::{thread::sleep, time::Duration};

use server::HttpRequests;

macro_rules! unwrap_or_panic {
    ($expr: expr) => {
        match $expr{
            Ok(res) =>{
                res
            },
                Err(e) => {panic!("Couldn't connect to server {}", e)
            }
        }   
    };
}

#[tokio::main]
async fn main() {
    const URL: &'static str  = "http://127.0.0.1:5000";
    let params = [("OS", "HI")];

    let server = server::Connection::new(&URL);

    let id = unwrap_or_panic!(server.register(&params).await);

    println!("{}",id);


    // let res = unwrap_or_panic!(server.register());

    // tokio::spawn(async move {
    //     loop {
    //         server.becon().await;
    //         sleep(Duration::from_secs(10));
    //     }
        
    // });

    
}
