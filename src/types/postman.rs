#![allow(dead_code)]

use std::fmt;

use serde::{Deserialize, Serialize};
use serde_json::Value;

//NOTE the Collection type is generated in reference to
//https://schema.postman.com/collection/json/v2.1.0/draft-07/docs/index.html

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Collection {
    info: Information,
    pub item: Vec<Items>,
    event: Option<Value>,
    variable: Option<Value>,
    protocolProfileBehavior: Option<Value>,
}

#[derive(Deserialize)]
struct Information {
    name: String,
    r#_postman_id: Option<String>,
    description: Option<Value>,
    version: Option<Value>,
    schema: Option<url::Url>, //TODOP this is a mandatory field, validate url deseralization and
                              //update type to mandatory
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Items {
    Item(Item),
    Folder(Folder),
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Folder {
    pub name: Option<String>,
    description: Option<Value>,
    variable: Option<Value>,
    pub item: Vec<Items>,
    event: Option<Vec<Value>>,
    pub auth: Option<Auth>,
    protocolProfileBehavior: Option<Value>,
}

//TODO convert the below structure to an enum so that while generating auth headers no looping is
//needed instead normal struct get would work
#[derive(Deserialize, Clone)]
pub struct Auth {
    pub r#type: AuthType,
    noauth: Option<Value>,
    apikey: Option<Vec<AuthAttr>>,
    awsv4: Option<Vec<AuthAttr>>,
    pub basic: Option<Vec<AuthAttr>>,
    pub bearer: Option<Vec<AuthAttr>>,
    digest: Option<Vec<AuthAttr>>,
    edgegrid: Option<Vec<AuthAttr>>,
    hawk: Option<Vec<AuthAttr>>,
    ntlm: Option<Vec<AuthAttr>>,
    oauth1: Option<Vec<AuthAttr>>,
    oauth2: Option<Vec<AuthAttr>>,
}

#[derive(Deserialize, Clone)]
pub struct AuthAttr {
    pub key: String,
    pub value: AuthAttrValue,
    r#type: String,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum AuthAttrValue {
    String(String),
    Number(i32),
}

impl fmt::Display for AuthAttrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(s) => write!(f, "{}", s),
            Self::Number(i) => write!(f, "{}", i),
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Deserialize, Clone, Debug)]
pub enum AuthType {
    String(String),
    apikey,
    awsv4,
    basic,
    bearer,
    digest,
    edgegrid,
    hawk,
    noauth,
    oauth1,
    oauth2,
    ntlm,
}

#[derive(Deserialize)]
#[allow(non_snake_case)]
pub struct Item {
    id: Option<String>,
    pub name: Option<String>,
    description: Option<Value>,
    variable: Option<Value>,
    event: Option<Value>,
    pub request: Request,
    response: Option<Vec<Value>>,
    protocolProfileBehavior: Option<Value>,
}

//TODOP this might be a good candidate for custom deseralization
//If a string, the string is assumed to be the request URL and the method is assumed to be 'GET'.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum Request {
    String(String),
    RequestStruct(RequestStruct),
}

#[derive(Deserialize)]
pub struct RequestStruct {
    pub url: Url,
    pub auth: Option<Auth>,
    proxy: Option<Value>,
    certificate: Option<Value>,
    pub method: Method,
    description: Option<Value>,
    pub header: Vec<Header>,
    pub body: Option<Body>,
}

//NOTE according to postman doc below is correct, though can't think of a case where header value
//would be a string :think
//#[derive(Deserialize)]
//#[serde(untagged)]
//enum Header {
//    String(String),
//    HeaderList(Vec<HeaderStruct>),
//}

#[derive(Deserialize, Clone)]
pub struct Header {
    pub key: String,
    pub value: String,
    //TODO disabled should ideally be just a boolean field, write custom
    //deseralization to accomplish this
    pub disabled: Option<bool>,
    pub description: Option<Value>,
}

//TODOP this again is a good candidate for custom deseralization
//If object, contains the complete broken-down URL for this request.
//If string, contains the literal request URL.
#[derive(Deserialize)]
#[serde(untagged)]
pub enum Url {
    String(String),
    UrlStruct(UrlStruct),
}

#[derive(Deserialize)]
pub struct UrlStruct {
    //The string representation of the request URL, including the protocol, host, path, hash, query parameter(s) and path variable(s).
    pub raw: String,
    protocol: Option<String>,
    host: Option<Host>,
    path: Option<Path>,
    port: Option<String>,
    pub query: Option<Vec<QueryParam>>,
    hash: Option<String>,
    variable: Option<Vec<Value>>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Host {
    String(String),
    VecStr(Vec<String>),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Path {
    String(String),
    ArrString(Vec<String>),
}

#[derive(Deserialize)]
pub struct QueryParam {
    pub key: Option<String>,
    pub value: Option<String>,
    pub disabled: Option<bool>,
    description: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug)]
//#[serde(untagged)]
pub enum Method {
    GET,
    PUT,
    POST,
    PATCH,
    DELETE,
    COPY,
    HEAD,
    OPTIONS,
    LINK,
    UNLINK,
    PURGE,
    LOCK,
    UNLOCK,
    PROPFIND,
    VIEW,
    //    String(String),
}

#[derive(Deserialize)]
pub struct Body {
    pub mode: BodyMode,
    pub raw: Option<String>,
    graphql: Option<Value>,
    urlencoded: Option<Vec<UrlEncodedParam>>,
    pub formdata: Option<Value>,
    file: Option<File>,
    pub options: Option<BodyOptions>,
    pub disabled: Option<bool>,
}

#[derive(Deserialize)]
pub struct BodyOptions {
    pub raw: BodyLanguageStruct
}

#[derive(Deserialize)]
pub struct BodyLanguageStruct {
    pub language: BodyLanguage
}

#[derive(Deserialize, PartialEq)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum BodyLanguage {
    Text,
    Javascript,
    Json,
    Html,
    Xml
}

impl fmt::Display for BodyLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let content_type = match self {
            Self::Text => "text/plain",
            Self::Javascript => "application/javascript",
            Self::Json => "application/json",
            Self::Html => "text/html",
            Self::Xml => "application/xml"
        };
        write!(f, "{}", content_type)
    }
}

#[derive(Deserialize)]
struct File {
    src: Option<String>,
    content: Option<String>,
}

#[derive(Deserialize)]
struct UrlEncodedParam {
    key: String,
    value: Option<String>,
    disabled: Option<bool>,
    description: Option<Value>,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
pub enum BodyMode {
    raw,
    urlencoded,
    formdata,
    file,
    graphql,
}
