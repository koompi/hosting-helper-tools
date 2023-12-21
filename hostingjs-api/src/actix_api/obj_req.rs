use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct ThemesData {
    server_name: String,
    theme_link: String,
    env: Value,
    files: Vec<FilesData>,
}

impl ThemesData {
    pub fn get_env(&self) -> HashMap<String, String> {
        let binding = serde_json::Map::new();
        let obj = &self.env.as_object().unwrap_or(&binding);
        obj.iter()
            .map(|(each_key, each_val)| {
                (each_key.to_string(), each_val.as_str().unwrap().to_string())
            })
            .collect::<HashMap<String, String>>()
    }

    pub fn get_theme_link(&self) -> &String {
        &self.theme_link
    }
    pub fn get_server_name(&self) -> &String {
        &self.server_name
    }
    pub fn get_files(&self) -> &Vec<FilesData> {
        &self.files
    }
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
