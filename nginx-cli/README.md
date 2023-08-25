# NGINX-CLI

NGINX HELPER TOOLS is CLI tools to use libnginx-wrapper to create safe and easy nginx configuration with SSL HTTPS; NGINX HELP TOOLS needs sudo;

```
Nginx Helper Tools

Usage: nginx-cli <COMMAND>

Commands:
  add, -a, --add        Add new NGINX configuration file
  delete, -D, --delete  Delete NGINX configuration file
  list, -l, --list      list NGINX configuration file
  force, -F, --force    Force reConfigure NGINX or Certbot Program
  help                  Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

## Add

```
Add new NGINX configuration file

Usage: nginx-cli {add|--add|-a} --proxy --redir --filehost --spa --dname <domain_name> --target <target>...

Options:
  -p, --proxy                Proxy Feature: Add Reverse Proxy NGINX configuration file
  -r, --redir                Redirect Feature: Add Redirect NGINX configuration file
  -f, --filehost             FileHost Feature: Add File Hosting NGINX configuration file
  -s, --spa                  SPA Feature: Add Single Page App NGINX configuration file
  -d, --dname <domain_name>  Domain Name to receive ReverseProxy/Redirect/SPA/FileHost request from; eg: koompi.com
  -t, --target <target>...   Domain name to ReverseProxy/Redirect/SPA/FileHost to; eg: http://localhost:8080 or https://koompi.app or /kmp/filehost-spa
  -h, --help                 Print help
  ```

## Delete

```Delete NGINX configuration file

Usage: nginx-cli {delete|--delete|-D} --dname <domain_name>

Options:
  -d, --dname <domain_name>  Delete by Domain Name
  -h, --help                 Print help
```

## List

```
list NGINX configuration file

Usage: nginx-cli {list|--list|-l} [OPTIONS] --proxy --redir --filehost --spa --all --one

Options:
  -p, --proxy                List NGINX Object of Proxy Feature
  -r, --redir                List NGINX Object of Redirect Feature
  -f, --filehost             List NGINX Object of FileHost Feature
  -s, --spa                  List NGINX Object of SPA Feature
  -a, --all                  List NGINX Object of All Feature
  -o, --one                  List one NGINX Object
  -d, --dname <domain_name>  Domain Name to receive ReverseProxy/Redirect/SPA/FileHost request from; eg: koompi.com
  -h, --help                 Print help
```

## Force

```
Force reConfigure NGINX or Certbot Program

Usage: nginx-cli {force|--force|-F} [OPTIONS] --cert --migration

Options:
  -c, --cert                 Renew Certificate: Force Certbot renew certificate for domain name NGINX configuration file
  -d, --dname <domain_name>  Domain Name to force redo certificate
  -m, --migration            Database Migration: Force Repopulate DB with configuration file 
  -h, --help                 Print help
```