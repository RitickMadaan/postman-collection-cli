use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use super::postman::{Method::{*, self}, RequestStruct, Url, AuthType, AuthAttr, Auth, Header, Request};
use std::{str::FromStr, string::String, result};

type Result<T> = result::Result<T, String>;

fn get_reqw_builder(mut client: reqwest::Client, mut url: Url, method: Method) -> Result<reqwest::RequestBuilder> {
    let url = match url {
        Url::String(raw) => raw,
        Url::UrlStruct(u) => u.raw,
    };
    //TODO try this error case once
    let unssuported_method_err = Err(format!("unsupported {:?}", method));
    let mut reqw = match method {
        GET => client.get(url),
        PUT => client.put(url),
        POST => client.post(url),
        PATCH => client.post(url),
        DELETE => client.delete(url),
        HEAD => client.head(url),
        _ => return unssuported_method_err,
    };
    Ok(reqw)
}

fn set_reqw_headers(mut reqw: reqwest::RequestBuilder, req_headers: Vec<Header>) -> Result<reqwest::RequestBuilder> {
    let mut headers = HeaderMap::new();
    for h in req_headers.into_iter() {
        let key = HeaderName::from_str(h.key.as_str())
            .map_err(|_| format!("header key: {}'s parsing failed", h.key))?;
        let value = HeaderValue::from_str(h.value.as_str())
            .map_err(|_| format!("header value: {}'s parsing failed", h.value))?;
        headers.insert(key, value);
    }
    reqw = reqw.headers(headers);
    Ok(reqw)
}

fn set_reqw_basic_auth(attrs: Vec<AuthAttr>, mut reqw: reqwest::RequestBuilder) -> Result<reqwest::RequestBuilder> {
    let username = attrs.clone().into_iter().filter(|a| a.key == "username").next().ok_or("basic auth authorization username not found")?.value;
    let pass = attrs.into_iter().filter(|a| a.key == "password").next().map(|p| p.value);
    Ok(reqw.basic_auth(username, pass))
}

fn set_reqw_bearer_auth(attrs: Vec<AuthAttr>, mut reqw: reqwest::RequestBuilder) -> Result<reqwest::RequestBuilder> {
    let token = attrs.into_iter().filter(|a| a.key == "token").next().ok_or("bearer authorization token not found")?.value;
    Ok(reqw.bearer_auth(token))
}

fn set_reqw_auth(mut reqw: reqwest::RequestBuilder, mut auth1: Option<Auth>) -> Result<reqwest::RequestBuilder> {
    let auth = match auth1 {
        Some(a) => a,
        None => return Ok(reqw),
    };
    let result_reqw = match auth.r#type {
        AuthType::basic => {
            let attrs = auth.basic.ok_or(String::from("attributes not found for basic auth"))?;
            set_reqw_basic_auth(attrs, reqw)
        },
        AuthType::bearer => {
            let attrs = auth.bearer.ok_or(String::from("attributes not found for bearer auth"))?;
            set_reqw_bearer_auth(attrs, reqw)
        }, 
        //TODO try the below error case
        auth_type  => return Err(format!("unsupported : {auth_type:?}")),
    };
    result_reqw
}

impl Request {
    //TODO separate out struct_to_reqwest fxn to multiple fxns
    fn struct_to_reqwest(
        client: reqwest::Client,
        req: RequestStruct,
    ) -> Result<reqwest::RequestBuilder> {
        let mut reqw = get_reqw_builder(client, req.url, req.method)?;
        reqw = set_reqw_headers(reqw, req.header)?;
        reqw = set_reqw_auth(reqw, req.auth)?; //TODO
        Ok(reqw)
    }

    pub fn to_reqwest(self) -> Result<reqwest::RequestBuilder> {
        let reqw_client = reqwest::Client::new();
        match self {
            Request::String(url) => Ok(reqw_client.get(url)),
            Request::RequestStruct(req) => Self::struct_to_reqwest(reqw_client, req),
        }
    }
}
