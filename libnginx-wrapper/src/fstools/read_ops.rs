use super::{
    super::{PROXY_SITES_PATH, REDIRECT_SITES_PATH},
    read_dir, BufReader, NginxFeatures, NginxObj, OpenOptions, Read,
};

pub(crate) fn read_nginx_dir() -> Vec<NginxObj> {
    let mut ngx_obj_vec = read_nginx_from_dir(PROXY_SITES_PATH, NginxFeatures::Proxy);
    ngx_obj_vec.append(&mut read_nginx_from_dir(
        REDIRECT_SITES_PATH,
        NginxFeatures::Redirect,
    ));
    ngx_obj_vec
}

fn read_nginx_from_dir(nginx_path: &str, feat_type: NginxFeatures) -> Vec<NginxObj> {
    read_dir(nginx_path)
        .unwrap()
        .map(|each| {
            let tmp = each.unwrap().path();
            let source_file = tmp.as_os_str().to_str().unwrap();
            match feat_type {
                NginxFeatures::Proxy => extract_nginx_proxy(read_file(source_file)),
                NginxFeatures::Redirect => extract_nginx_redirect(read_file(source_file)),
                NginxFeatures::SPA => {
                    extract_nginx_filehost_and_spa(read_file(source_file), NginxFeatures::SPA)
                }
                NginxFeatures::FileHost => {
                    extract_nginx_filehost_and_spa(read_file(source_file), NginxFeatures::FileHost)
                }
            }
        })
        .collect::<Vec<NginxObj>>()
}

fn extract_nginx_proxy(config: String) -> NginxObj {
    let mut proxy_pass = String::new();
    let mut server_name = String::new();
    config.lines().for_each(|each_line| {
        let each_line = each_line.trim();
        if each_line.contains("proxy_pass") {
            proxy_pass = each_line
                .split_whitespace()
                .last()
                .unwrap()
                .replace(";", "");
        } else if each_line.contains("server_name") {
            server_name = each_line
                .split_whitespace()
                .last()
                .unwrap()
                .replace(";", "");
        }
    });

    NginxObj::new(server_name, proxy_pass, NginxFeatures::Proxy)
}

fn extract_nginx_redirect(config: String) -> NginxObj {
    let mut target_site = String::new();
    let mut server_name = String::new();
    config.lines().for_each(|each_line| {
        let each_line = each_line.trim();
        if each_line.contains("rewrite") {
            target_site = each_line
                .trim()
                .replace("rewrite ^/(.*)$ ", "")
                .replace("/$1 permanent;", "");
        } else if each_line.contains("server_name") {
            server_name = each_line
                .split_whitespace()
                .last()
                .unwrap()
                .replace(";", "");
        }
    });

    NginxObj::new(server_name, target_site, NginxFeatures::Redirect)
}

fn extract_nginx_filehost_and_spa(config: String, feature: NginxFeatures) -> NginxObj {
    let mut target_location = String::new();
    let mut server_name = String::new();
    config.lines().for_each(|each_line| {
        let each_line = each_line.trim();
        if each_line.contains("root") {
            target_location = each_line
                .split_whitespace()
                .last()
                .unwrap()
                .replace(";", "");
        } else if each_line.contains("server_name") {
            server_name = each_line
                .split_whitespace()
                .last()
                .unwrap()
                .replace(";", "");
        }
    });

    NginxObj::new(server_name, target_location, feature)
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
