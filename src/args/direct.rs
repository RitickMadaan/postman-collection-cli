use std::fmt::Display;

use crate::{types::postman::{Collection, Request}, utils::get_req_from_coll_item};
use serde::Serialize;
use serde_json::json;
use reqwest::RequestBuilder;


impl Collection {
    async fn process(self, path: String) -> Result<reqwest::Response, String> {
        let pman_req = get_req_from_coll_item(self.item, &path.split("/").collect())?;
        let reqw = pman_req.to_reqwest()?;
        reqw.send().await.map_err(|e| e.to_string())//TODO create a json response type
    }

    pub async fn direct(self, path: String) {
        match self.process(path).await {
            Ok(resp) => println!("{:#?}", resp),
            Err(e) => println!("{e}"), //TODO exit with code 1 in this case
        }
    }
}

