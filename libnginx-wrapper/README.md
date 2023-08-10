# libnginx-wrapper

## Object Available

- http_server::NginxObj 
  - new(server_name, target_site, feature) -> Result\<NginxObj>
  - finish(self) -> Result

## Function Available

- init_migration(force)
- http_server::remove_nginx_conf(server_name) -> Result
- db::select_all_from_tbl_nginxconf() -> Vec\<NginxObj>
- db::select_all_by_feature_from_tbl_nginxconf(feature) -> Vec\<NginxObj>
- http_server::remake_ssl(server_name) -> Result

