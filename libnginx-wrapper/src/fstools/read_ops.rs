use super::{read_dir, BufReader, NginxObj, OpenOptions, Read};

pub fn read_nginx_dir() -> Vec<NginxObj> {
    read_dir("/etc/nginx/sites-enabled")
        .unwrap()
        .map(|each| {
            let tmp = each.unwrap().path();
            let source_file = tmp.as_os_str().to_str().unwrap();
            extract_nginx_obj(read_file(source_file))
        })
        .collect::<Vec<NginxObj>>()
}

fn extract_nginx_obj(config: String) -> NginxObj {
    let mut proxy_pass = String::new();
    let mut server_name = String::new();
    config
        .lines()
        .filter(|each| each.contains("proxy_pass") || each.contains("server_name"))
        .for_each(|each| match each {
            "proxy_pass" => proxy_pass = each.split_whitespace().last().unwrap().replace(";", ""),
            "server_name" => server_name = each.split_whitespace().last().unwrap().replace(";", ""),
            _ => {}
        });
    NginxObj::new(server_name, proxy_pass).unwrap()
}

fn read_file(source_file: &str) -> String {
    match OpenOptions::new().read(true).open(source_file) {
        Ok(file_read) => {
            let mut read_buffer = BufReader::new(&file_read);
            let mut contents: Vec<u8> = Vec::new();
            read_buffer.read_to_end(&mut contents).unwrap();
            String::from_utf8_lossy(&contents).to_string()
        }
        Err(_error) => String::new(),
    }
}
