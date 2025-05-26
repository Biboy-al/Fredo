mod server;
use server::HttpRequests;

macro_rules! unwrap_or_panic {
    ($expr: expr) => {
        match $expr{
            Ok(res) =>{
                res
            },
                Err(e) => {panic!("oh no {}", e)
            }
        }   
    };
}

fn main() {


    let server = server::Server{url:"http://127.0.0.1:5000/registry"};

    let res = unwrap_or_panic!(server.register());

    print!("{}", res);
}
