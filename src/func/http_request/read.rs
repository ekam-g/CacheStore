use crate::func::http_request::Request;
use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MainParse {
    pub error: String,
}
#[derive(Serialize, Deserialize)]
pub struct Internal {
    pub error: String,
    pub data: Vec<String>,
}

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
    pub async fn read_more(url: String) -> Result<Internal, Error> {
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
