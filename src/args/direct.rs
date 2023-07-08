use std::collections::HashMap;

use crate::{
    types::postman::Collection,
    utils::get_req_from_coll_item,
};
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json::{json, Value};
use url::Url;

#[derive(Serialize)]
struct ProcessedResponse {
    status: u16,
    http_version: String,
    headers: HashMap<String, HeaderValues>,
    content_length: Option<u64>,
    url: Url,
    body: Value,
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
enum HeaderValues {
    Str(String),
    Vec(Vec<String>),
}

fn serialize_headermap(headermap: &HeaderMap) -> Result<HashMap<String, HeaderValues>, String> {
    let mut headers: HashMap<String, HeaderValues> = HashMap::new();
    for (name, val) in headermap {
        let value = val.to_str().map_err(|e| e.to_string())?.to_string();
        match headers.clone().get_mut(name.as_str()) {
            Some(HeaderValues::Str(x)) => {
                headers.insert(
                    name.to_string(),
                    HeaderValues::Vec(vec![x.to_owned(), value]),
                );
            }
            Some(HeaderValues::Vec(x)) => x.push(value),
            None => {
                headers.insert(name.to_string(), HeaderValues::Str(value));
            }
        }
    }
    Ok(headers)
}

impl Collection {
    async fn process(self, path: String) -> Result<ProcessedResponse, String> {
        let pman_req = get_req_from_coll_item(self.item, &path.split("/").collect())?;
        let reqw = pman_req.to_reqwest()?;
        let resp = reqw.send().await.map_err(|x| x.to_string())?;
        let resp_headers = serialize_headermap(resp.headers())?;
        let mut processed_resp = ProcessedResponse {
            status: resp.status().as_u16(),
            http_version: format!("{:?}", resp.version()),
            headers: resp_headers,
            content_length: resp.content_length(),
            url: resp.url().to_owned(),
            body: Value::Null,
        };
        processed_resp.body = resp
            .text()
            .await
            .map_or(Value::Null, |text| text.parse().unwrap_or(text.into()));

        Ok(processed_resp)
    }

    pub async fn direct(self, path: String) {
        match self.process(path).await {
            Ok(resp) => println!("{:#}", json!(resp)),
            Err(e) => println!("{e}"), //TODO exit with code 1 in this case
        }
    }
}
