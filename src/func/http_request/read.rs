use reqwest::{Error};
use crate::func::http_request::Request;
use serde::{Serialize, Deserialize};



//this may need to be changed
#[derive(Serialize, Deserialize)]
pub struct MainParse {
    pub data: String,
}
//end

impl Request {
    pub async fn read() -> Result<MainParse, Error> {
        let output = reqwest::Client::new()
            .get("Your URL")
            .send()
            .await;
        return match output {
            Ok(data) => {
                let final_data = data.json()
                    .await;
                match final_data {
                    Ok(yes) => {
                        Ok(yes)
                    }
                    Err(error) => {
                        Err(error)
                    }
                }
            }
            Err(err) => {
                Err(err)
            }
        };
    }
}