use crate::{types::postman::{Collection, Request}, utils::get_req_from_coll_item};
use serde_json::json;
use reqwest::RequestBuilder;


impl Collection {
    pub fn direct(self, path: String) {
        let postman_request = get_req_from_coll_item(self.item, &path.split("/").collect());
        let reqw_client = reqwest::Client::new();
//        let reqw = match postman_request {
//            String(url) => reqw_client.get(url),
//            Request(req) => 
//        }
        println!("Request = {}", json!({}));
    }
}

//impl FromPostman<RequestBuilder> for RequestBuilder {
//    fn from_postman_req(req: Request) -> RequestBuilder {
//        let reqw_client = reqwest::Client::new();
//        reqw_client.get("https://google.com")
//    }
//}
