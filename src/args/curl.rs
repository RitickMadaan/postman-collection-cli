use crate::{types::postman::Collection, utils::get_req_from_coll_item};
use dbg_as_curl::*;
use reqwest::{Request, RequestBuilder};

impl Collection {
    pub fn get_curl(self, path: String) {
        get_req_from_coll_item(self.item, &path.split("/").collect())
            .and_then(|postman_req| postman_req.to_reqwest())
            .and_then(|reqw_builder: RequestBuilder| {
                reqw_builder.build().map_err(|e| {
                    format!(
                        "Failed to create Request from RequestBuilder with err: {}",
                        e
                    )
                })
            })
            .map(|req: Request| format!("{}", AsCurl::new(&req)))
            .map(|c| println!("{c}"))
            .unwrap_or_else(|e| panic!("{e}")); //TODO in case of error exit with code 1
    }
}
