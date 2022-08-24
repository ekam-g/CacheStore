use std::iter::Map;
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


impl Request{
   pub async fn read() -> MainParse {
      let data: MainParse = reqwest::Client::new()
          .get("http://localhost:3000/getAll?apiKey=gdsjahbcadyfasdcnlhdfgaoDSHANFDIUGAYSDFAWSNDdifuHDBSFJDSFHSDOUAGHNHBOUhgougfUYFoulhbygcIOpihnijBuygfouboiuHouhvouGHFVOIUlniohgOUHvouVY9TDIHGvihgvifhgcd56476")
          .send()
          .await.expect("failed on request")
          .json()
          .await.expect("failed on json");
      return data;
   }
}