# HOSTING-HELPER-TOOLS

HOST-HELPER-TOOLS is program to expose safe NGINX usage to REST API and CLI. NginxModule has 2 apps and 3 library:

- nginx-cli
- nginx-api
- libnginx-wrapper
- libcloudflare-wrapper
- libdatabase

## NGINX API

NGINX-API is written in RUST Actix-web. NGINX-API expose [API List](nginx-api/README.md)

## NGINX-CLI

NGINX-CLI is written in RUST CLAP-RS. NGINX-CLI [Help](nginx-cli/README.md)

coming soon!

## LibNGINX-Wrapper

Libnginx-wrapper is written in Rust std. Libnginx-wrapper expose 1 Object with 3 public builtin functions, and 5 public free function as shown in [List](libnginx-wrapper/README.md)

## LibCloudflare-Wrapper

Libcloudflare-wrapper is written in Rust std and Reqwest. Libcloudflare-wrapper expose 2 public functions as shown in [List](libcloudflare-wrapper/README.md)

## LibDatabase

libdatabase is written in Rust std and SQLITE Database. Libnginx-wrapper expose 2 public free function as shown in [List](libdatabase/README.md)