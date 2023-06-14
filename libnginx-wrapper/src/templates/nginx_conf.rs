pub(crate) fn gen_templ() -> String {
    format!("
user www-data;
worker_cpu_affinity auto;
worker_processes auto;
pid /run/nginx.pid;
include /etc/nginx/modules-enabled/*.conf;

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

    include /etc/nginx/mime.types;
    default_type application/octet-stream;

    ##
    # SSL Settings
    ##

    ssl_protocols TLSv1 TLSv1.1 TLSv1.2 TLSv1.3; # Dropping SSLv3, ref: POODLE
    ssl_prefer_server_ciphers on;

    ##
    # Logging Settings
    ##

    access_log /var/log/nginx/access.log;
    error_log /var/log/nginx/error.log;

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
    include /etc/nginx/conf.d/*.conf;
    include /etc/nginx/sites-enabled/*.conf;
    server {{
        listen      80 default_server;
        server_name _;
        return      444;
    }}
    server {{
        listen 443 ssl;
        server_name _;
        ssl_certificate /etc/nginx/ssl/nginx.crt;
        ssl_certificate_key /etc/nginx/ssl/nginx.key;
        return       444;
    }}

}}

stream {{
    log_format basic '$remote_addr [$time_local] '
                     '$protocol $status $bytes_sent $bytes_received '
                     '$session_time';

    access_log  /var/log/nginx/access.log basic;
    error_log  /var/log/nginx/error.log debug;
    
    include /etc/nginx/sites-stream/*.conf;
}}")
}