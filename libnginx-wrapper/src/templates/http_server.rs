pub(crate) fn gen_proxy_templ(proxy_pass: &str, server_name: &str) -> String {
    let proxy_host = url::Url::parse(proxy_pass).unwrap();
    let proxy_host = proxy_host.domain().unwrap();
    format!(
        r#"
server {{
    server_name {server_name};
    location / {{
        proxy_pass {proxy_pass};
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
        r#"
server {{
    server_name {server_name};
    location / {{
        rewrite ^/(.*)$ {target_site}/$1 permanent;
    }}

    listen [::]:80;
    listen 80;

}}"#
    )
}
