use super::super::SPECIAL_WEBSITE;

pub(crate) fn gen_proxy_templ(
    proxy_pass: Vec<String>,
    server_name: &str,
    protocol: &str,
) -> String {
    let upstream_server = proxy_pass
        .iter()
        .map(|each| format!("\tserver {};", each.replace(protocol, "").replace("://", "")))
        .collect::<Vec<String>>()
        .join("\n");
    let proxy_host = proxy_pass
        .into_iter()
        .filter(|each| {
            let url_pre = url::Url::parse(&each).unwrap();
            let url_parsed = url_pre.domain().unwrap();
            SPECIAL_WEBSITE.iter().any(|each| each.contains(url_parsed))
        })
        .next()
        .unwrap_or(String::from("$host"));

    let server_name_id = server_name.replace(['.', '-'], "");

    format!(
        r#"upstream {server_name_id} {{
{upstream_server}
}}

server {{
    server_name {server_name};
    location / {{
        proxy_pass {protocol}://{server_name_id};
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host {proxy_host};
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }}

    listen [::]:80;
    listen 80;

}}"#
    )
}

pub(crate) fn gen_redirect_templ(target_site: &str, server_name: &str) -> String {
    format!(
        r#"server {{
    server_name {server_name};
    location / {{
        rewrite ^/(.*)$ {target_site}/$1 permanent;
    }}

    listen [::]:80;
    listen 80;

}}"#
    )
}

pub(crate) fn gen_filehost_templ(target_location: &str, server_name: &str) -> String {
    format!(
        r#"server {{
    server_name {server_name};
    location / {{
        root {target_location};
        autoindex on;
        sendfile           on;
        sendfile_max_chunk 1m;  
        tcp_nopush on;
        keepalive_timeout 65;
    }}

    listen [::]:80;
    listen 80;

}}"#
    )
}

pub(crate) fn gen_spa_templ(target_location: &str, server_name: &str) -> String {
    format!(
        r#"server {{
    server_name {server_name};
    root {target_location};
    index index.html index.htm index.nginx-debian.html;
    location / {{
        try_files $uri /index.html;
        #try_files $uri $uri/ =404;
    }}

    listen [::]:80;
    listen 80;

}}"#
    )
}
