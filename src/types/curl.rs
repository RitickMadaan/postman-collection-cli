use serde_json::Value;

use crate::types::postman::{
    Auth, Body, BodyLanguage, BodyMode, BodyOptions, Header, Request, RequestStruct, Url,
};
use base64::Engine;
use std::fmt;

pub struct Curl(pub Request);

impl fmt::Display for Curl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "curl --location")?;
        let RequestStruct {
            method,
            url,
            auth,
            header,
            body,
            ..
        }: &RequestStruct = match self {
            Curl(Request::String(url)) => return write!(f, " {url}"),
            Curl(Request::RequestStruct(r)) => r,
        };

        //TODO rename the struct type itself to headers instead of header by using serde
        //attributes
        //TODO THINK is the cloning here really worth it ? or shall we go without pushing the
        //vector approach and writing on the spot for headers (anyways doing the same in write_body)
        let mut headers: Vec<Header> = header.to_vec();

        let url = match url {
            Url::String(raw) => raw,
            Url::UrlStruct(u) => &u.raw,
        };

        if let Some(auth) = auth {
            let auth_header = add_auth_headers(auth).unwrap();
            headers.push(auth_header);
        }

        write!(f, " --request {method:?}")?;
        write!(f, " '{url}'")?;

        headers
            .into_iter()
            .filter(|h| h.disabled == None)
            .map(|h| write!(f, " \\\n--header '{}: {}'", h.key, h.value))
            .collect::<Result<Vec<()>, _>>()?;

        if let Some(body) = body {
            write_body(f, body)?
        }
        Ok(())
    }
}

fn add_auth_headers(auth: &Auth) -> Result<Header, String> {
    match auth {
        Auth {
            basic: Some(basic_auth_params),
            ..
        } => {
            let mut authorization = String::from(":");
            basic_auth_params.into_iter().for_each(|p| {
                //TODO below code doesn't mandate the following params while it should, should be
                //rfactored by making Auth type an enum of different Authorization types
                match p.key.as_str() {
                    "password" => { 
                        authorization = format!("{authorization}{}", p.value);
                    },
                    "username" => authorization = format!("{}{authorization}", p.value),
                    _ => (),
                };
            });
            authorization = format!(
                "Basic {}",
                base64::engine::general_purpose::STANDARD.encode(authorization.as_str())
            );
            Ok(Header {
                key: String::from("Authorization"),
                value: authorization,
                description: None,
                disabled: None,
            })
        }
        _ => Err(String::from("unssuported auth type"))
    }
}

fn write_body(f: &mut fmt::Formatter<'_>, body: &Body) -> fmt::Result {
    match body {
        Body {
            mode: BodyMode::raw,
            raw: Some(raw),
            options,
            ..
        } => {
            let mut data_arg = String::from("--data-raw");
            match options {
                Some(BodyOptions { raw }) if raw.language != BodyLanguage::Json => {
                    data_arg = String::from("--data");
                    //TODO move this header as well to be appended the headers vector itself
                    write!(f, " \\\n--header 'Content-Type: {}'", raw.language)?
                }
                _ => (),
            };
            let raw_data = match serde_json::from_str::<Value>(raw) {
                Ok(raw) => format!("{:#}", raw),
                Err(_) => raw.to_owned(),
            };
            write!(f, " \\\n{data_arg} '{}'", raw_data)?
        }
        _ => panic!("Unsupported body type"),
        //urlencoded => (),
        //formdata => (),
        //file => (),
        //graphql => (),
    };

    Ok(())
}
