pub fn gen_templ(proxy_pass: &str, server_name: &str) -> String {
    format!(
        r#"
server {{
    server_name {};
    location / {{
        proxy_pass {};
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection "upgrade";
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-Host $host;
        proxy_set_header X-Forwarded-Port $server_port;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }}

    listen [::]:80;
    listen 80;

}}    
"#, server_name, proxy_pass
    )
}
