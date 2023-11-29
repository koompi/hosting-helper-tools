use std::collections::HashMap;

// use super::{bytes::Bytes, text::Text, MultipartForm};
use serde::Deserialize;
use serde_json::Value;

// #[derive(MultipartForm)]
// pub struct ThemeInfo {
//     theme_link: Text<String>,
//     server_name: Text<String>,
//     files: Vec<Bytes>,
// }

// impl ThemeInfo {
//     pub fn get_theme_link(&self) ->  &String {
//         &self.theme_link
//     }
//     pub fn get_server_name(&self) ->  &String {
//         &self.server_name
//     }
//     pub fn get_files(&self) ->  &Vec<Bytes> {
//         &self.files
//     }

// }

#[derive(Deserialize)]
pub struct ThemesData {
    server_name: String,
    theme_link: String,
    env: Value,
    files: Vec<FilesData>,
}

impl ThemesData {
    pub fn get_env(&self) -> HashMap<String, String> {
        let obj = &self
            .env
            .as_object()
            .unwrap();
        obj
            .iter()
            .map(|(each_key, each_val)| (each_key.to_string(), each_val.as_str().unwrap().to_string()))
            .collect::<HashMap<String, String>>()
    }

    pub fn get_theme_link(&self) ->  &String {
        &self.theme_link
    }
    pub fn get_server_name(&self) ->  &String {
        &self.server_name
    }
    pub fn get_files(&self) -> &Vec<FilesData> {
        &self.files
    }

    // pub fn get_file(&self) -> &Vec<FilesData> {
    //     &self
    //         .files
    //         .as_array()
    //         .unwrap()
    //         .iter()
    //         .map(|each| serde_json::from_value::<FilesData>(*each).unwrap())
    //         .collect::<Vec<FilesData>>()
    // }
}

#[derive(Deserialize)]
pub struct FilesData {
    filename: String,
    path: Option<String>,
    data: Value,
}
impl FilesData {
    pub fn get_filename(&self) -> &String {
        &self.filename
    }
    pub fn get_path(&self) -> &Option<String> {
        &self.path
    }
    pub fn get_data(&self) -> &Value {
        &self.data
    }
}