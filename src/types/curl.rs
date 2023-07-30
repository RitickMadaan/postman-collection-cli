use serde_json::Value;

use crate::types::postman::{
    Body, BodyLanguage, BodyMode, BodyOptions, Request, RequestStruct, Url,
};
use std::fmt;

pub struct Curl(pub Request);

//fn write_raw_body()

impl fmt::Display for Curl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "curl --location")?;
        let RequestStruct {
            method,
            url,
            header,
            body,
            ..
        }: &RequestStruct = match self {
            Curl(Request::String(url)) => return write!(f, " {url}"),
            Curl(Request::RequestStruct(r)) => r,
        };

        write!(f, " --request {method}")?;

        let url = match url {
            Url::String(raw) => raw,
            Url::UrlStruct(u) => &u.raw,
        };

        //TODO add authorization

        write!(f, " '{url}'")?;

        header
            .into_iter()
            .filter(|h| h.disabled == None)
            .map(|h| write!(f, " \\\n--header '{}: {}'", h.key, h.value))
            .collect::<Result<Vec<()>, _>>()?;

        match body {
            Some(Body {
                mode: BodyMode::raw,
                raw: Some(raw),
                options,
                ..
            }) => {
                match options {
                    Some(BodyOptions { raw }) if raw.language != BodyLanguage::Json => {
                        write!(f, " \\\n--header 'Content-Type: {}", raw.language)?
                    },
                    _ => (),
                };
                let raw_data = match serde_json::from_str::<Value>(raw) {
                    Ok(raw) => format!("{:#}", raw),
                    Err(_) => raw.to_owned(),
                };
                write!(f, " \\\n--data-raw '{}'", raw_data)?;
            }
            _ => (),
            //urlencoded => (),
            //formdata => (),
            //file => (),
            //graphql => (),
        };

        Ok(())
    }
}
