use reqwest::{self, Client};
use chrono::{self, Utc};

#[derive(serde::Serialize)]
pub struct SendData{
    sent: String
}

#[derive(serde::Deserialize)]
pub struct RecData{
    rec: String,
}

pub struct Connection<'a>{
    url: &'a str,
    reg: &'a str,
    becon: &'a str,
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

impl<'a> Connection<'a>{
    pub fn new(url : &'a str) -> Connection<'a>{

        Connection{
            url:url,
            reg: "/register",
            becon: "/becon",
            server: reqwest::Client::new()
        }
    }


    //the only one that should use a sync
    pub async fn register(&self, os:& str) -> Result<String, reqwest::Error> {
        let params = [("OS", os)];
        let url = format!("{}{}",self.url,self.reg);

        let response = self.server.post(url)
        .form(&params)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    pub async fn becon(&self, id:& str) -> Result<String, reqwest::Error> {
        let params = [("id", id), ("timestamp", &chrono::Utc::now().to_string())];
        let url = format!("{}{}",self.url,self.becon);
        let response = self.server.get(url)
        .form(&params)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    // pub fn post_request(&self) -> Result<RecData, ureq::Error>{

    //     let send_body = SendData {sent: "yo".to_string()};

    //     let body:RecData = ureq::post(self.url)
    //         .header("example-Header", "Header Value")
    //         .send_json(&send_body)?
    //         .body_mut()
    //         .read_json::<RecData>()?;

    //     Ok(body)

    // }

    // pub fn get_request(&self) -> Result<String, ureq::Error> {

    //     let body: String = ureq::get(self.url)
    //     .header("Example-Header", "header value")
    //     .call()?
    //     .body_mut()
    //     .read_to_string()?;
    //     Ok(body)
    // }
    
}
