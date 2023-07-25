use reqwest as reqw;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::json;

use super::postman::{
    Auth, AuthAttr, AuthType, Body, BodyMode, Header,
    Method::{self, *},
    Request, RequestStruct, Url, UrlStruct,
};
use std::{collections::HashMap, result, str::FromStr, string::String};

type Result<T> = result::Result<T, String>;

fn get_reqw_builder(
    client: &reqwest::Client,
    url: &Url,
    method: Method,
) -> Result<reqwest::RequestBuilder> {
    let url = match url {
        Url::String(raw) => raw,
        Url::UrlStruct(u) => &u.raw,
    };
    //TODO try this error case once
    let unssuported_method_err = Err(format!("unsupported {:?}", method));
    let reqw = match method {
        GET => client.get(url),
        PUT => client.put(url),
        POST => client.post(url),
        PATCH => client.patch(url),
        DELETE => client.delete(url),
        HEAD => client.head(url),
        _ => return unssuported_method_err,
    };
    Ok(reqw)
}

fn set_reqw_headers(
    mut reqw: reqwest::RequestBuilder,
    req_headers: Vec<Header>,
) -> Result<reqwest::RequestBuilder> {
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

fn set_reqw_basic_auth(
    reqw: reqwest::RequestBuilder,
    attrs: Vec<AuthAttr>,
) -> Result<reqwest::RequestBuilder> {
    let username = json!(
        attrs
            .clone() //TODO can be removed ?
            .into_iter()
            .filter(|a| a.key == "username")
            .next()
            .ok_or("basic auth authorization username not found")?
            .value
    );
    let pass = attrs
        .into_iter()
        .filter(|a| a.key == "password")
        .next()
        .map(|p| json!(p.value));
    //NOTE not converting the values attribute values to Value leads to stack overflow
    Ok(reqw.basic_auth(username, pass))
}

fn set_reqw_bearer_auth(
    reqw: reqwest::RequestBuilder,
    attrs: Vec<AuthAttr>,
) -> Result<reqwest::RequestBuilder> {
    let token = attrs
        .into_iter()
        .filter(|a| a.key == "token")
        .next()
        .ok_or("bearer authorization token not found")?
        .value;
    Ok(reqw.bearer_auth(token))
}

fn set_reqw_auth(
    reqw: reqwest::RequestBuilder,
    auth: Option<Auth>,
) -> Result<reqwest::RequestBuilder> {
    let auth = match auth {
        Some(a) => a,
        None => return Ok(reqw),
    };
    let result_reqw = match auth.r#type {
        AuthType::basic => {
            let attrs = auth
                .basic
                .ok_or(String::from("attributes not found for basic auth"))?;
            set_reqw_basic_auth(reqw, attrs)
        }
        AuthType::bearer => {
            let attrs = auth
                .bearer
                .ok_or(String::from("attributes not found for bearer auth"))?;
            set_reqw_bearer_auth(reqw, attrs)
        }
        //TODO try the below error case
        auth_type => return Err(format!("unsupported : {auth_type:?}")),
    };
    result_reqw
}

fn set_reqw_body_or_form(
    reqw: reqwest::RequestBuilder,
    body: Option<Body>,
) -> Result<reqwest::RequestBuilder> {
    let body = match body {
        Some(b) if b.disabled == Some(true) => return Ok(reqw),
        Some(b) => b,
        _ => return Ok(reqw),
    };
    match body.mode {
        BodyMode::raw => Ok(reqw.body(body.raw.ok_or(String::from("raw body not found"))?)),
        BodyMode::formdata => {
            Ok(reqw.form(&body.formdata.ok_or(String::from("formdata not found"))?))
        }
        _ => Err(String::from("unsupported body mode")),
    }
}

fn set_reqw_query_params(
    reqw: reqwest::RequestBuilder,
    url: Url,
) -> Result<reqwest::RequestBuilder> {
    let query = match url {
        Url::UrlStruct(UrlStruct { query: Some(q), .. }) => q,
        _ => return Ok(reqw),
    };
    let params = query.into_iter().fold(HashMap::new(), |mut map, q| {
        if q.disabled == Some(true) {
            return map;
        }
        if let (Some(key), Some(val)) = (q.key, q.value) {
            map.insert(key, val);
        }
        map
    });
    Ok(reqw.query(&params))
}

//NOTE all the above fxns can be part of a trait. In future if there is a need to generate request
//in formats other than curl/RequestBuilder then move to traits itself.
impl Request {
    fn struct_to_reqwest(
        client: &reqwest::Client,
        req: RequestStruct,
    ) -> Result<reqwest::RequestBuilder> {
        let mut reqw = get_reqw_builder(&client, &req.url, req.method)?;
        reqw = set_reqw_headers(reqw, req.header)?;
        reqw = set_reqw_auth(reqw, req.auth)?;
        reqw = set_reqw_body_or_form(reqw, req.body)?;
        reqw = set_reqw_query_params(reqw, req.url)?;
        Ok(reqw)
    }

    pub fn to_reqwest(self) -> Result<(reqw::Client, reqw::Request)> {
        let reqw_client = reqwest::Client::new();

        let reqw = match self {
            Request::String(url) => reqw_client.get(url).build(),
            Request::RequestStruct(req) => Self::struct_to_reqwest(&reqw_client, req)?.build(),
        }
        .map_err(|e| e.to_string())?;

        Ok((reqw_client, reqw))
    }
}
