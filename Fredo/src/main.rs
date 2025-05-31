mod server;
use std::{thread::sleep, time::Duration};


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

    let server = server::Connection::new(&URL);

    let id: String = unwrap_or_panic!(server.register("HI").await);

    tokio::spawn(async move {
        loop {
            server.becon(&id).await;
            sleep(Duration::from_secs(10));
        }
    });

    loop{

    }
    
}
