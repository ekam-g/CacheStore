use std::io;
use std::iter::Map;
use reqwest::{Error, Response};
use crate::func::http_request::Request;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MainParse {
    pub(crate) data: Vec<Embed>,
}

#[derive(Serialize, Deserialize)]
pub struct Embed {
    pub(crate) perm: String,
}


impl Request {
    pub async fn read() -> Result<MainParse, Error> {
        let output = reqwest::Client::new()
            .get("http://localhost:3000/getAll?apiKey=gdsjahbcadyfasdcnlhdfgaoDSHANFDIUGAYSDFAWSNDdifuHDBSFJDSFHSDOUAGHNHBOUhgougfUYFoulhbygcIOpihnijBuygfouboiuHouhvouGHFVOIUlniohgOUHvouVY9TDIHGvihgvifhgcd56476")
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