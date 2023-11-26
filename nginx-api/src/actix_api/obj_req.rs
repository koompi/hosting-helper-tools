use super::{bytes::Bytes, text::Text, MultipartForm};

#[derive(MultipartForm)]
pub struct ThemeInfo {
    theme_link: Text<String>,
    server_name: Text<String>,
    files: Vec<Bytes>,
}

impl ThemeInfo {
    pub fn get_theme_link(&self) ->  &String {
        &self.theme_link
    }    
    pub fn get_server_name(&self) ->  &String {
        &self.server_name
    }
    pub fn get_files(&self) ->  &Vec<Bytes> {
        &self.files
    }    

}