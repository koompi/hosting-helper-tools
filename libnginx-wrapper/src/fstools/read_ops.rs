use super::{
    super::{PROXY_SITES_PATH, REDIRECT_SITES_PATH},
    read_dir, BufReader, NginxFeatures, NginxObj, OpenOptions, Read, TargetSite,
};

pub(crate) fn read_nginx_dir() -> Vec<NginxObj> {
    let mut ngx_obj_vec = Vec::new();

    ngx_obj_vec.append(&mut read_nginx_from_dir(
        PROXY_SITES_PATH,
        NginxFeatures::Proxy,
    ));
    ngx_obj_vec.append(&mut read_nginx_from_dir(
        REDIRECT_SITES_PATH,
        NginxFeatures::Redirect,
    ));
    ngx_obj_vec.append(&mut read_nginx_from_dir(
        REDIRECT_SITES_PATH,
        NginxFeatures::SPA,
    ));
    ngx_obj_vec.append(&mut read_nginx_from_dir(
        REDIRECT_SITES_PATH,
        NginxFeatures::FileHost,
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
                _ => unreachable!(),
            }
        })
        .collect::<Vec<NginxObj>>()
}

fn extract_nginx_proxy(config: String) -> NginxObj {
    // let mut proxy_pass = String::new();
    let mut server_name = String::new();
    let mut protocol: String = String::new();
    let mut upstream_data: Vec<String> = Vec::new();
    let mut upstream_detected: Option<bool> = None;
    config
        .lines()
        .map(|each_line| each_line.trim())
        .for_each(|each_line| {
            // Block of Code for filtering upstream server
            {
                if let Some(true) = upstream_detected {
                    // println!("Line: {each_line}");
                    if each_line == "}" {
                        upstream_detected = Some(false);
                    } else {
                        upstream_data.push(
                            each_line
                                .split_ascii_whitespace()
                                .last()
                                .unwrap()
                                .replace(";", ""),
                        );
                    }
                }
            }

            if upstream_detected == None {
                if each_line.starts_with("upstream") {
                    upstream_detected = Some(true);
                }
            } else if each_line.starts_with("server_name") {
                server_name = each_line
                    .split_whitespace()
                    .last()
                    .unwrap()
                    .replace(";", "");
            } else if each_line.starts_with("proxy_pass") {
                protocol = each_line
                    .replace("proxy_pass", "")
                    .trim()
                    .split("://")
                    .next()
                    .unwrap()
                    .to_string();
            }
        });

    {
        upstream_data = upstream_data
            .into_iter()
            .map(|each| format!("{protocol}://{each}"))
            .collect();
        let server_name = server_name;
        let target_site = match upstream_data.len() {
            1 => TargetSite::Single(upstream_data.into_iter().next().unwrap()),
            _ => TargetSite::Multiple(upstream_data),
        };
        let feature = NginxFeatures::Proxy;
        NginxObj::new(server_name, target_site, feature).unwrap()
    }
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

    {
        let server_name = server_name;
        let target_site = TargetSite::Single(target_site);
        let feature = NginxFeatures::Redirect;
        NginxObj::new(server_name, target_site, feature).unwrap()
    }
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

    {
        let server_name = server_name;
        let target_site = TargetSite::Single(target_location);
        let feature = feature;
        NginxObj::new(server_name, target_site, feature).unwrap()
    }
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
