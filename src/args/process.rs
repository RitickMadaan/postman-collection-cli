use std::collections::HashMap;

use crate::{types::postman::Collection, utils::get_req_from_coll_item};
use reqwest::header::HeaderMap;
use serde::Serialize;
use serde_json::{json, Value};
use url::Url;

#[derive(Serialize)]
struct ProcessResp {
    request: Req,
    response: Resp,
}

#[derive(Serialize)]
struct Resp {
    status: u16,
    headers: HashMap<String, HeaderValues>,
    body: Value,
}

#[derive(Serialize)]
struct Req {
    url: Url,
    method: String,
    headers: HashMap<String, HeaderValues>,
    body: Option<Value>,
    http_version: String,
    timeout_in_secs: Option<u64>, //TODO see how it appears in output, if not good then change to
                                  //string and format to seconds yourself using debug formatting maybe
                                  //?
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
    async fn process(self, path: String) -> Result<ProcessResp, String> {
        let postman_req = get_req_from_coll_item(self.item, &path.split("/").collect())?;
        let (reqw_client, reqw) = postman_req.to_reqwest()?;
        let result_req = Req {
            url: reqw.url().to_owned(),
            method: reqw.method().to_string(),
            body: reqw
                .body()
                .and_then(|b| b.as_bytes())
                .and_then(|b| String::from_utf8(b.to_owned()).ok())
                .and_then(|b| b.parse().ok()),
            headers: serialize_headermap(reqw.headers())?,
            http_version: format!("{:?}", reqw.version()),
            timeout_in_secs: reqw.timeout().map(|d| d.as_secs()),
        };
        let reqw_resp = reqw_client.execute(reqw).await.map_err(|x| x.to_string())?;
        let mut result_resp = Resp {
            status: reqw_resp.status().as_u16(),
            headers: serialize_headermap(reqw_resp.headers())?,
            body: Value::Null,
        };
        result_resp.body = reqw_resp
            .text()
            .await
            .map_or(Value::Null, |text| text.parse().unwrap_or(text.into()));
        Ok(ProcessResp {
            request: result_req,
            response: result_resp,
        })
    }

    pub async fn direct(self, path: String) {
        match self.process(path).await {
            Ok(resp) => println!("{:#}", json!(resp)),
            Err(e) => println!("{e}"), //TODO exit with code 1 in this case
        }
    }
}
