mod server;
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

fn main() {


    let server = server::Server{
        url:"http://127.0.0.1:5000",
        reg: "/registry",
        becon: "/becon"
    };

    let res = unwrap_or_panic!(server.register());
    
    print!("{}", res);
}
