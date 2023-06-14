# libnginx-wrapper

## Object Available

- http_server::NginxObj 
  - new(server_name, proxy_pass) -> NginxObj
  - finish(self) -> Result

## Function Available

- init_migration(force)
- http_server::remove_nginx_conf -> Result
- db::select_all_from_tbl_nginxconf -> Vec\<NginxObj>

