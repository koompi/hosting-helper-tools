# nginx-modules

NginxModules is program to expose safe NGINX usage to REST API and CLI. NginxModule has 3 apps:

- nginx-cli
- nginx-api
- libnginx-wrapper

## NGINX API

NGINX-API is written in RUST Actix-web. NGINX-API expose [API List](nginx-api/README.md)

## NGINX-CLI

NGINX-CLI is written in RUST CLAP-RS. NGINX-CLI [Help](nginx-cli/README.md)

coming soon!

## LibNGINX-Wrapper

Libnginx-wrapper is written in Rust std and SQLITE Database. Libnginx-wrapper expose 1 Object with 3 public builtin functions, and 4 public free function as shown in [List](libnginx-wrapper/README.md)