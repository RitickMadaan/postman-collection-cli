use serde_json::Value;
use serde::Deserialize;

//NOTE the Collection type is generated in reference to
//https://schema.postman.com/collection/json/v2.1.0/draft-07/docs/index.html

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize)]
pub struct Collection {
    info: Information,
    item: Items,
    event: Option<Value>,
    variable: Option<Value>,
    protocolProfileBehavior: Option<Value>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct Information {
    name: String,
    _postman_id: Option<String>,
    description: Option<Value>,
    version: Option<Value>,
    schema: Option<url::Url> //TODOP this is a mandatory field, validate url deseralization and
                             //update type to mandatory
}

#[derive(Deserialize)]
pub enum Items {
    Item(Item),
    Folder
}

#[derive(Deserialize)]
#[allow(dead_code, non_snake_case)]
pub struct Item {
    id: Option<String>,
    name: Option<String>,
    description: Option<Value>,
    variable: Option<Value>,
    event: Option<Value>,
    request: Request,
    response: Vec<Value>,//TODOP add type for this...
    protocolProfileBehavior: Option<Value>
}

//TODOP this might be a good candidate for custom deseralization
//If a string, the string is assumed to be the request URL and the method is assumed to be 'GET'.
#[derive(Deserialize)]
pub enum Request {
    String(String),
    Value(Value)//TODOP correct this
}

pub struct RequestStruct {
    url: Url,
    auth: Option<Value>,//TODOP add type for this
    proxy: Option<Value>,//TODOP ^
    certificate: Option<Value>,//TODOP ^
    method: MethodEnum,
    description: Option<Value>,
    header: Header,
}

pub enum Header {
    String(String),
    HeaderList(Vec<HeaderStruct>)
}

pub struct HeaderStruct {
    key: String,
    value: String,
    disabled: Option<bool>,
    description: Option<Value>,
    body: Option<Value>//TODOP add type for this
}

//TODOP this again is a good candidate for custom deseralization
//If object, contains the complete broken-down URL for this request.
//If string, contains the literal request URL.
pub enum Url {
   String(String),
   Value(Value)//TODOP correct this
}

pub struct UrlStruct {
    //The string representation of the request URL, including the protocol, host, path, hash, query parameter(s) and path variable(s).
    raw: String,
    protocol: String,
    host: Host,
    path: Path,
    port: String,
    query: Vec<QueryParam>,
    hash: String,
    variable: [Value]
}

pub enum Host {
    String(String),
    ArrString(Vec<String>),
}

pub enum Path {
    String(String),
    ArrString(Vec<String>)
}

pub struct QueryParam {
    key: Option<String>,
    value: Option<String>,
    disabled: bool,
    description: Value
}


pub enum MethodEnum {
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
    String(String)
}
