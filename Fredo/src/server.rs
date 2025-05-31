use reqwest::{self, Client};

#[derive(serde::Serialize)]
pub struct SendData{
    sent: String
}

#[derive(serde::Deserialize)]
pub struct RecData{
    rec: String,
}

pub struct Connection{
    url: &'static str,
    reg: &'static str,
    becon: &'static str,
    server: reqwest::Client
}

macro_rules! unwrap_or_panic {
    ($expr: expr) => {
        match $expr{
            Ok(res) =>{
                res
            },
                Err(e) => {panic!("Couldn't connect to server with an error: {}", e)
            }
        }   
    };
}

impl Connection{
    pub fn new(url : &'static str) -> Connection{

        Connection{
            url:url,
            reg: "/register",
            becon: "/becon",
            server: reqwest::Client::new()
        }
    }
}

pub trait HttpRequests{
    async fn register(&self, params: &[(&'static str, &'static str)]) -> Result<String, reqwest::Error>;
    async fn becon(&self) -> bool;
    fn post_request(&self) -> Result<RecData, ureq::Error>;
    fn get_request(&self) -> Result<String, ureq::Error>;
}

macro_rules! craft_req {
    ($expr:expr) => {
        ureq::get($expr)
        .header("Example-Header", "header value")
        .call()?
        .body_mut()
        .read_to_string()?
    };
}

impl HttpRequests for Connection{


    //the only one that should use a sync
    async fn register(&self, params: &[(&'static str, &'static str)]) -> Result<String, reqwest::Error> {

        let url = format!("{}{}",self.url,self.reg);

        let request = self.server.post(url)
        .form(&params)
        .send().await?;

        Ok(request.text().await?)
    }

    async fn becon(&self) -> bool {
        let url = format!("{}{}",self.url,self.becon);
        match reqwest::get(url).await{
            Ok(_) => true,
            Err(_) => false
        }
    }

    fn post_request(&self) -> Result<RecData, ureq::Error>{

        let send_body = SendData {sent: "yo".to_string()};

        let body:RecData = ureq::post(self.url)
            .header("example-Header", "Header Value")
            .send_json(&send_body)?
            .body_mut()
            .read_json::<RecData>()?;

        Ok(body)

    }

    fn get_request(&self) -> Result<String, ureq::Error> {

        let body: String = ureq::get(self.url)
        .header("Example-Header", "header value")
        .call()?
        .body_mut()
        .read_to_string()?;
        Ok(body)
    }
    
}

