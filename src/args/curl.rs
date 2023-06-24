use crate::types::postman::Collection;

impl Collection {
    pub fn get_curl(&self, path: String) {
        println!("path received for curl: {path}");
    }
}
