use crate::func::http_request::Request;
use reqwest::Error;
use serde::{Deserialize, Serialize};

//this may need to be changed
#[derive(Serialize, Deserialize)]
pub struct MainParse {
    pub error: String,
}
//end

impl Request {
    pub async fn read(url: String) -> Result<MainParse, Error> {
        let output = reqwest::Client::new().get(url).send().await;
        return match output {
            Ok(data) => {
                let final_data = data.json().await;
                match final_data {
                    Ok(yes) => Ok(yes),
                    Err(error) => Err(error),
                }
            }
            Err(err) => Err(err),
        };
    }
}
