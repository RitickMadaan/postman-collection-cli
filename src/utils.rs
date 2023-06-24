use crate::types::postman::{Folder, Item, Items, Items::*, Request};

fn get_req_from_items(items: Items, path: &Vec<&str>) -> Result<Request, String> {
    let item_name = path.get(0).ok_or("Invalid request path")?;
    let err = Err(format!("{item_name} not found"));
    match (items, path.len()) {
        (Item(Item { name, request, .. }), 1) if name == Some(item_name.to_string()) => Ok(request),
        (
            Folder(Folder {
                name,
                item: folder_item,
                ..
            }),
            1..,
        ) if name == Some(item_name.to_string()) => {
            get_req_from_coll_item(folder_item, &path[1..].to_vec())
        }
        _ => err,
    }
}

pub fn get_req_from_coll_item(
    folder_item: Vec<Items>,
    path: &Vec<&str>,
) -> Result<Request, String> {
    let mut req = Err(String::from("not found"));
    for items in folder_item.into_iter() {
        req = get_req_from_items(items, path);
        if req.is_ok() {
            break;
        }
    }
    req
}
