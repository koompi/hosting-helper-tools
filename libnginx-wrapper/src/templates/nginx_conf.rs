use super::super::{SPA_SITES_PATH, FILE_SITES_PATH, PROXY_SITES_PATH, STREAM_SITES_PATH, REDIRECT_SITES_PATH, PROGRAM_BASE_NAME, NGINX_DEFAULT_CERT_PATH};

pub(crate) fn gen_templ() -> String {
    format!("user root;
worker_cpu_affinity auto;
worker_processes auto;
pid /run/{PROGRAM_BASE_NAME}.pid;
include /etc/{PROGRAM_BASE_NAME}/modules-enabled/*.conf;

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

    include /etc/{PROGRAM_BASE_NAME}/mime.types;
    default_type application/octet-stream;

    ##
    # SSL Settings
    ##

    ssl_protocols TLSv1 TLSv1.1 TLSv1.2 TLSv1.3; # Dropping SSLv3, ref: POODLE
    ssl_prefer_server_ciphers on;

    ##
    # Logging Settings
    ##

    access_log /var/log/{PROGRAM_BASE_NAME}/access.log;
    error_log /var/log/{PROGRAM_BASE_NAME}/error.log;

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
    include /etc/{PROGRAM_BASE_NAME}/conf.d/*.conf;
    include {PROXY_SITES_PATH}/*.conf;
    include {REDIRECT_SITES_PATH}/*.conf;
    include {FILE_SITES_PATH}/*.conf;
    include {SPA_SITES_PATH}/*.conf;
    server {{
        listen      80 default_server;
        server_name _;
        return      444;
    }}
    server {{
        listen 443 ssl;
        server_name _;
        ssl_certificate {NGINX_DEFAULT_CERT_PATH}/{PROGRAM_BASE_NAME}.crt;
        ssl_certificate_key {NGINX_DEFAULT_CERT_PATH}/{PROGRAM_BASE_NAME}.key;
        return       444;
    }}

}}

stream {{
    log_format basic '$remote_addr [$time_local] '
                     '$protocol $status $bytes_sent $bytes_received '
                     '$session_time';

    access_log  /var/log/{PROGRAM_BASE_NAME}/access.log basic;
    error_log  /var/log/{PROGRAM_BASE_NAME}/error.log debug;
    
    include {STREAM_SITES_PATH}/*.conf;
}}")
}