#![allow(dead_code)]
use serde::Deserialize;
use serde_json::Value;

//NOTE the Collection type is generated in reference to
//https://schema.postman.com/collection/json/v2.1.0/draft-07/docs/index.html

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Collection {
    info: Information,
    item: Vec<Items>,
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
enum Items {
    Item(Item),
    Folder(Folder),
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Folder {
    name: Option<String>,
    description: Option<Value>,
    variable: Option<Value>,
    item: Vec<Items>,
    event: Option<Vec<Value>>,
    auth: Option<Auth>,
    protocolProfileBehavior: Option<Value>,
}

#[derive(Deserialize)]
struct Auth {
    r#type: AuthType,
    noauth: Option<Value>,
    apikey: Option<Vec<AuthAttr>>,
    awsv4: Option<Vec<AuthAttr>>,
    basic: Option<Vec<AuthAttr>>,
    bearer: Option<Vec<AuthAttr>>,
    digest: Option<Vec<AuthAttr>>,
    edgegrid: Option<Vec<AuthAttr>>,
    hawk: Option<Vec<AuthAttr>>,
    ntlm: Option<Vec<AuthAttr>>,
    oauth1: Option<Vec<AuthAttr>>,
    oauth2: Option<Vec<AuthAttr>>,
}

#[derive(Deserialize)]
struct AuthAttr {
    key: String,
    value: Option<Value>,
    r#type: String,
}

#[allow(non_camel_case_types)]
#[derive(Deserialize)]
enum AuthType {
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
struct Item {
    id: Option<String>,
    name: Option<String>,
    description: Option<Value>,
    variable: Option<Value>,
    event: Option<Value>,
    request: Request,
    response: Option<Vec<Value>>,
    protocolProfileBehavior: Option<Value>,
}

//TODOP this might be a good candidate for custom deseralization
//If a string, the string is assumed to be the request URL and the method is assumed to be 'GET'.
#[derive(Deserialize)]
#[serde(untagged)]
enum Request {
    String(String),
    RequestStruct(RequestStruct),
}

#[derive(Deserialize)]
struct RequestStruct {
    url: Url,
    auth: Option<Auth>,
    proxy: Option<Value>,
    certificate: Option<Value>,
    method: Method,
    description: Option<Value>,
    header: Header,
    body: Option<Body>,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Header {
    String(String),
    HeaderList(Vec<HeaderStruct>),
}

#[derive(Deserialize)]
struct HeaderStruct {
    key: String,
    value: String,
    disabled: Option<bool>,
    description: Option<Value>,
}

//TODOP this again is a good candidate for custom deseralization
//If object, contains the complete broken-down URL for this request.
//If string, contains the literal request URL.
#[derive(Deserialize)]
#[serde(untagged)]
enum Url {
    String(String),
    UrlStruct(UrlStruct),
}

#[derive(Deserialize)]
struct UrlStruct {
    //The string representation of the request URL, including the protocol, host, path, hash, query parameter(s) and path variable(s).
    raw: String,
    protocol: Option<String>,
    host: Host,
    path: Option<Path>,
    port: Option<String>,
    query: Option<Vec<QueryParam>>,
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
struct QueryParam {
    key: Option<String>,
    value: Option<String>,
    disabled: bool,
    description: Value,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Method {
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
    String(String),
}

#[derive(Deserialize)]
struct Body {
    mode: Option<BodyMode>,
    raw: String,
    graphql: Option<Value>,
    urlencoded: Option<Vec<UrlEncodedParam>>,
    formdata: Option<Value>,
    file: Option<File>,
    options: Option<Value>,
    disabled: Option<bool>,
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
enum BodyMode {
    raw,
    urlencoded,
    formdata,
    file,
    graphql,
}
