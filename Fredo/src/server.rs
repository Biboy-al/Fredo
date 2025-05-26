

#[derive(serde::Serialize)]
pub struct SendData{
    sent: String
}

#[derive(serde::Deserialize)]
pub struct RecData{
    rec: String
}

pub struct Server{
    pub url: &'static str
}

pub trait HttpRequests{
    fn register(&self) -> Result<String, ureq::Error>;
    fn post_request(&self) -> Result<RecData, ureq::Error>;
    fn get_request(&self) -> Result<String, ureq::Error>;
}

impl HttpRequests for Server{

    fn register(&self) -> Result<String, ureq::Error> {

        let body: String = ureq::get(self.url)
        .header("Example-Header", "header value")
        .call()?
        .body_mut()
        .read_to_string()?;
        print!("{}", body);
        Ok(body)
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

