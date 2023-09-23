use crate::types::postman::{Collection, Folder, Item, Items, Items::*, Request};
use std::*;

fn get_req_from_items(items: Items, path: &Vec<String>) -> Result<Request, String> {
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
    path: &Vec<String>,
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

fn get_collections_from_current_dir() -> io::Result<Vec<Collection>> {
    let read_dir = fs::read_dir(env::current_dir()?.as_path())?;
    let mut collections = Vec::new();
    read_dir.for_each(|item| {
        let item = match item {
            Ok(i) => i,
            Err(_) => return (),
        };
        let file_type = item.file_type();
        let file_name = item.file_name().into_string();
        let json_file_regex = regex::Regex::new(r"\.json$");
        match (file_type, file_name, json_file_regex) {
            (Ok(file_type), Ok(file_name), Ok(json_regex))
                if !file_type.is_dir() && json_regex.is_match(file_name.as_str()) =>
            {
                let collection: Collection = match fs::read_to_string(item.path())
                    .map(|f| serde_json::from_str(f.as_str()))
                {
                    Ok(Ok(coll)) => coll,
                    _ => return (),
                };
                collections.push(collection)
            }
            _ => return (),
        }
    });
    Ok(collections)
}

pub fn get_req_from_current_dir(path: &Vec<String>) -> Result<Request, String> {
    let (collection_name, path) = path.split_first().ok_or(String::from("invalid path"))?;
    //    path = vec!(path.to_owned());
    for collection in get_collections_from_current_dir()
        .map_err(|e| e.to_string())?
        .into_iter()
    {
        if collection.info.name == collection_name.to_owned() {
            match get_req_from_coll_item(collection.item, &path.to_vec()) {
                Ok(req) => return Ok(req),
                _ => continue,
            }
        }
    }
    Err(String::from("not found"))
}

fn get_items_path_list(items: Vec<Items>, current_path: String, items_path_list: &mut Vec<String>) {
    let get_item_path = |item_name: Option<String>| {
        format!(
            "{current_path}/{}",
            item_name.unwrap_or(String::from("no_name"))
        )
    };
    for item in items.into_iter() {
        match item {
            Items::Item(item) => items_path_list.push(get_item_path(item.name)),
            Items::Folder(folder) => {
                get_items_path_list(folder.item, get_item_path(folder.name), items_path_list)
            }
        }
    }
}

pub fn reqs_paths_in_current_dir() -> Result<Vec<String>, String> {
    let collections = get_collections_from_current_dir().map_err(|e| e.to_string())?;
    let mut reqs_path_list = Vec::new();
    for collection in collections.into_iter() {
        let current_path = collection.info.name;
        let items = collection.item;
        get_items_path_list(items, current_path, &mut reqs_path_list);
    }
    Ok(reqs_path_list)
}

pub fn copy_to_clipbaord(x: impl std::fmt::Display) -> Result<(), arboard::Error> {
    let mut clipboard = arboard::Clipboard::new()?;
    clipboard.set_text(format!("{x}"))
}
