use crate::{types::{postman::Collection, curl::Curl}, utils::get_req_from_coll_item};

impl Collection {
    pub fn get_curl(self, path: String) {
        match get_req_from_coll_item(self.item, &path.split("/").collect()) {
            Ok(req) => println!("{}", Curl(req)),
            Err(e) => println!("{e}"), //TODO exit with code 1 here
        }
    }
}
