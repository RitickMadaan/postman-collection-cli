use crate::types::postman::{Collection, Folder, Item, Items, Items::*, Request};
use std::*;

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

pub fn get_req_from_current_dir(path: &Vec<&str>) -> Result<Request, String> {
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

fn item_names_from_items(items: Vec<Items>) -> Vec<String> {
    let mut sub_paths = Vec::new();
    items.into_iter().for_each(|item| match item {
        Items::Item(Item { name, .. }) => sub_paths.push(name.unwrap_or(String::from(""))),
        Items::Folder(Folder { name, .. }) => sub_paths.push(name.unwrap_or(String::from(""))),
        _ => (),
    });
    sub_paths
}

fn get_items_with_name(item_name: &&str, items: Vec<Items>) -> Vec<Items> {
    items
        .into_iter()
        .filter(|item| match item {
            Items::Item(Item {
                name: Some(name), ..
            }) if &name.as_str() == item_name => true,
            Items::Folder(Folder {
                name: Some(name), ..
            }) if &name.as_str() == item_name => true,
            _ => false,
        })
        .collect()
}

pub fn next_sub_paths_in_collection_item(
    item: Items,
    path: &Vec<&str>,
) -> Result<Vec<String>, String> {
    match (path.split_first(), item) {
        (_, Items::Item(_)) => Ok(Vec::new()),
        (None, Items::Folder(Folder { item, .. })) => Ok(item_names_from_items(item)),
        (Some((sub_item_name, path)), Items::Folder(Folder { item: items, .. })) => {
            let matching_items = get_items_with_name(sub_item_name, items);
            Ok(matching_items
                .into_iter()
                .map(|item| {
                    next_sub_paths_in_collection_item(item, &path.to_vec()).unwrap_or(Vec::new())
                })
                .collect::<Vec<Vec<String>>>()
                .concat())
        }
    }
}

pub fn next_sub_path_in_collection(
    collection: Collection,
    path: &Vec<&str>,
) -> Result<Vec<String>, String> {
    let items = collection.item;
    match path.split_first() {
        None => Ok(item_names_from_items(items)),
        Some((item_name, path)) => {
            let items = get_items_with_name(item_name, items);
            Ok(items
                .into_iter()
                .map(|item| next_sub_paths_in_collection_item(item, &path.to_vec()))
                .collect::<Result<Vec<_>, String>>()?
                .concat())
        }
    }
}

pub fn get_next_sub_paths_in_curr_dir(path: &Vec<&str>) -> Result<Vec<String>, String> {
    let collections_in_curr_dir = get_collections_from_current_dir().map_err(|e| e.to_string())?;
    match path.split_first() {
        None => Ok(collections_in_curr_dir
            .into_iter()
            .map(|collection| collection.info.name)
            .collect()),
        Some((collection_name, req_path)) => Ok(collections_in_curr_dir
            .into_iter()
            .filter(|collection| collection.info.name == collection_name.clone())
            .map(|collection| {
                next_sub_path_in_collection(collection, &req_path.to_vec()).unwrap_or(Vec::new())
            })
            .collect::<Vec<Vec<String>>>()
            .concat()),
    }
}
