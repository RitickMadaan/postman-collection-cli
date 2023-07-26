use crate::{types::postman::Collection, utils::get_req_from_coll_item};

impl Collection {
    pub fn get_curl(self, path: String) {
        get_req_from_coll_item(self.item, &path.split("/").collect())
            .and_then(|postman_req| postman_req.to_curl())
            .map(|curl| println!("{curl}"))
            .unwrap_or_else(|e| println!("error: {e}"))//TODO exit with exit code 1 here
    }
}
