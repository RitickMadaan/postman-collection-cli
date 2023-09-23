use crate::types::curl::Curl;
use crate::types::postman::Request;
use crate::utils::{get_req_from_current_dir, reqs_paths_in_current_dir};

fn get_user_selected_req() -> Result<Request, String> {
    let reqs_paths_in_curr_dir = reqs_paths_in_current_dir()?;
    let prompt_str = "Select request from current directory:";
    let prompt_render_config = inquire::ui::RenderConfig::default();
    let selected_req_path = inquire::Select::new(prompt_str, reqs_paths_in_curr_dir)
        .with_render_config(prompt_render_config)
        .prompt()
        .map_err(|e| e.to_string())?;
    get_req_from_current_dir(&selected_req_path.split("/").map(|s| s.to_string()).collect())
}

pub fn get_curl() -> Result<Curl, String> {
    get_user_selected_req().map(|req| Curl(req))
}
