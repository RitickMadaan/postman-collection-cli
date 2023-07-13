use crate::{types::postman::Collection, utils::get_req_from_coll_item};
use dbg_as_curl::*;

impl Collection {
    pub fn get_curl(self, path: String) {
        get_req_from_coll_item(self.item, &path.split("/").collect())
            .and_then(|postman_req| postman_req.to_reqwest())
            .map(|(_reqw_client, reqw)| format!("{}", AsCurl::new(&reqw)))
            .map(|c| println!("{c}"))
            .unwrap_or_else(|e| panic!("{e}")); //TODO in case of error exit with code 1
    }
}
