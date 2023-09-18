pub(crate) fn gen_templ() -> String {
    let program_base_name = dotenv::var("PROGRAM_BASE_NAME").unwrap();
    let nginx_default_cert_path = dotenv::var("NGINX_DEFAULT_CERT_PATH").unwrap();
    let stream_sites_path = dotenv::var("STREAM_SITES_PATH").unwrap();
    let redirect_sites_path = dotenv::var("REDIRECT_SITES_PATH").unwrap();
    let proxy_sites_path = dotenv::var("PROXY_SITES_PATH").unwrap();
    let spa_sites_path = dotenv::var("SPA_SITES_PATH").unwrap();
    let file_sites_path = dotenv::var("FILE_SITES_PATH").unwrap();

    format!("user root;
worker_cpu_affinity auto;
worker_processes auto;
pid /run/{program_base_name}.pid;
include /etc/{program_base_name}/modules-enabled/*.conf;

events {{
    worker_connections 1024;
    multi_accept on;
}}

http {{

    ##
    # Basic Settings
    ##

    charset utf-8;
    sendfile on;
    tcp_nopush on;
    tcp_nodelay on;
    types_hash_max_size 4096;
    client_max_body_size 1024M;
    server_tokens off;

    # server_names_hash_bucket_size 64;
    # server_name_in_redirect off;

    include /etc/{program_base_name}/mime.types;
    default_type application/octet-stream;

    ##
    # SSL Settings
    ##

    ssl_protocols TLSv1 TLSv1.1 TLSv1.2 TLSv1.3; # Dropping SSLv3, ref: POODLE
    ssl_prefer_server_ciphers on;

    ##
    # Logging Settings
    ##

    access_log /var/log/{program_base_name}/access.log;
    error_log /var/log/{program_base_name}/error.log;

    ##
    # Gzip Settings
    ##

    gzip on;

    # gzip_vary on;
    # gzip_proxied any;
    # gzip_comp_level 6;
    # gzip_buffers 16 8k;
    # gzip_http_version 1.1;
    # gzip_types text/plain text/css application/json application/javascript text/xml application/xml application/xml+rss text/javascript;

    ##
    # Virtual Host Configs
    ##
    include /etc/{program_base_name}/conf.d/*.conf;
    include {proxy_sites_path}/*.conf;
    include {redirect_sites_path}/*.conf;
    include {file_sites_path}/*.conf;
    include {spa_sites_path}/*.conf;
    server {{
        listen      80 default_server;
        server_name _;
        return      444;
    }}
    server {{
        listen 443 ssl;
        server_name _;
        ssl_certificate {nginx_default_cert_path}/{program_base_name}.crt;
        ssl_certificate_key {nginx_default_cert_path}/{program_base_name}.key;
        return       444;
    }}

}}

stream {{
    log_format basic '$remote_addr [$time_local] '
                     '$protocol $status $bytes_sent $bytes_received '
                     '$session_time';

    access_log  /var/log/{program_base_name}/access.log basic;
    error_log  /var/log/{program_base_name}/error.log debug;
    
    include {stream_sites_path}/*.conf;
}}")
}