use std::iter::Map;
use crate::func::http_request::Request;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MainParse {
   data: Vec<Embed>,
}

#[derive(Serialize, Deserialize)]
pub struct Embed {
   perm: String,
}


impl Request{
   pub async fn read() -> MainParse {
      let data: Welcome = reqwest::Client::new()
          .get("http://localhost:3000/getAll?apiKey=gdsjahbcadyfasdcnlhdfgaoDSHANFDIUGAYSDFAWSNDdifuHDBSFJDSFHSDOUAGHNHBOUhgougfUYFoulhbygcIOpihnijBuygfouboiuHouhvouGHFVOIUlniohgOUHvouVY9TDIHGvihgvifhgcd56476")
          .send()
          .await.expect("failed on request")
          .json()
          .await.expect("failed on json");
      return data;
   }
}