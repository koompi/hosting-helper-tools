# NGINX-API

## How to run

- First, make file `settings.toml` from file `settings.toml.bk` at root directory of this project (login accounts need to be added to this file before running)

- Second, get binary from Compiling this project to the same place as `settings.toml`

- Next,
  ```bash
  sudo ./nginx-api
  ```

</details>

## Protected API

All protected API endpoint needs:

| Header      | Data Type |
| ----------- | --------- |
| X-Auth-User | String    |
| X-Auth-Pass | String    |

<details close="close">
<summary><b>GET</b> /nginx/list</summary>

---

| Header | Data Type |
| ------ | --------- |
| None   | None      |

Body

```json

```

Response 200

```json
{
  "code": 200,
  "message": [
    {
      "server_name": "tellsela.com",
      "target_site": "http://koompi.com",
      "feature": "Proxy"
    },
    {
      "server_name": "selatell.com",
      "target_site": "http://koompi.com",
      "feature": "Redirect"
    }
  ]
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

---

</details>

<details close="close">
<summary><b>POST</b> /nginx/add</summary>

---

| Header | Data Type |
| ------ | --------- |
| None   | None      |

Body

Use either appropriately

```json
{
  "server_name": "forwarder.koompi.com",
  "target_site": "http://localhost:8070",
  "feature": "Redirect"
}
```

**or**

```json
{
  "server_name": "forwarder.koompi.com",
  "target_site": [
    "http://localhost:8080",
    "http://localhost:8090",
    "http://localhost:8070"
  ],
  "feature": "Proxy"
}
```

| Variable    | Data Type                                                                                                     |
| ----------- | ------------------------------------------------------------------------------------------------------------- |
| server_name | String: eg. rithy.org                                                                                         |
| target_site | String: eg. https://weteka.org/user/rithy or String Array: ["http://localhost:3030", "http://localhost:2345"] |
| feature     | String: `Proxy` _or_ `Redirect` _or_ `FileHost` _or_ `SPA`                                                    |

| Query Paramter | Data Type                                                                                                             |
| -------------- | --------------------------------------------------------------------------------------------------------------------- |
| enom           | _true_ or _false_ for setting to configure enom or not: **default to true**                                           |
| ssl            | _true_ or _false_ for setting to configure ssl or not: **default to true**                                            |
| cloudflare     | _true_ or _false_ for setting to configure cloudflare: **default to true**                                            |
| ipcheck        | _true_ or _false_ for setting to check Public IP of the Domain in case Cloudflare is not enabled: **default to true** |

Link Example: `http://localhost:8080/nginx/add?enom=false&cloudflare=true&ssl=true`

Response 200

```json
{
  "code": 200,
  "message": "Ok"
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

- Note:
  - `THIS API TAKE LONG TIME`
  - `server_name` must be first DNS pointed to this nginx server IP before add, otherwise it will error certificate generation
  - `server_name` must not include SCHEMA and must not already existed
  - each item in `target_site` must be input in form of _SCHEMA://SUBDOMAIN.DOMAIN.TLD/WHATEVER_ (eg. http:// or https://) otherwise it will error _BADREQUEST_
  - `feature` is **ENUM of Proxy, Redirect, FileHost, and SPA** on the backend
  - definiton of each opiton in `feature`: `Proxy` (forward without changing name) _or_ `Redirect` (forward changing name) _or_ `FileHost` (host a file server) _or_ `SPA` (host single page application)

---

</details>

<details close="close">
<summary><b>POST</b> /cert/force/{server_name}</summary>

---

| Header | Data Type |
| ------ | --------- |
| None   | None      |

| Query Parameter | Data Type             |
| --------------- | --------------------- |
| server_name     | String: eg. rithy.org |

Body

```json

```

Response 200

```json
{
  "code": 200,
  "message": "Ok"
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

- Note:
  - This API is for forcing the _CERTBOT_ to redo certificate. This is actually not neccessary for main process, but only for troubleshooting TLS

---

</details>

<details close="close">
<summary><b>POST</b> /migration/force</summary>

---

| Header | Data Type |
| ------ | --------- |
| None   | None      |

Body

```json

```

Response 200

```json
{
  "code": 200,
  "message": "Ok"
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

- Note:
  - This API is for forcing the APP to rebuild Database in case of mismatch between file in directory and database rows

---

</details>

<details close="close">
<summary><b>POST</b> /nginx/update/{server_name}</summary>

---

| Header      | Data Type              |
| ----------- | ---------------------- |
| server_name | String; eg: koompi.com |

Body

```json
["http://localhost:8080", "http://localhost:8090", "http://localhost:8070"]
```

| Variable    | Data Type                                                                                                     |
| ----------- | ------------------------------------------------------------------------------------------------------------- |
| target_site | String: eg. https://weteka.org/user/rithy or String Array: ["http://localhost:3030", "http://localhost:2345"] |

Response 200

```json
{
  "code": 200,
  "message": "Ok"
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

---

</details>

<details close="close">
<summary><b>DELETE</b> /nginx/delete/{server_name}</summary>

---

| Header | Data Type |
| ------ | --------- |
| None   | None      |

| Query Parameter | Data Type             |
| --------------- | --------------------- |
| server_name     | String: eg. rithy.org |

Body

```json

```

Response 200

```json
{
  "code": 200,
  "message": "Ok"
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

- Note:
  - If `server_name` does not exist, it will error _BADREQUEST_

---

</details>

<details close="close">
<summary><b>POST</b> /hosting/add</summary>

---

| Header | Data Type |
| ------ | --------- |
| None   | None      |

Body

```json
{
  "server_name": "testingwebs23423.riverbase.org",
  "theme_link": "git@github.com:koompi/riverbase-themes.git",
  "env": {
    "VITE_VARIABLE_BACKEND": "http://localhost:8000",
    "VITE_VARIABLE_ID_STORE": "65677c4220d608fcf6eff114"
  },
  "files": [
    {
      "filename": "theme.json",
      "path": "themes",
      "data": {
        "color": {
          "primary": "#3674B7"
        }
      }
    }
  ]
}
```

| Variable    | Data Type                                                                 |
| ----------- | ------------------------------------------------------------------------- |
| server_name | String: eg. testingwebs23423.riverbase.org                                |
| theme_link  | String: eg. git@github.com:koompi/riverbase-themes.git                    |
| env         | Json Object Hashmap                                                       |
| files       | Json Object Array of filename, path and data of JSON Object without limit |

Response 200

```json
{
  "code": 200,
  "message": "Ok"
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

---

</details>

<details close="close">
<summary><b>PUT</b> /hosting/update</summary>

---

| Header | Data Type |
| ------ | --------- |
| None   | None      |

Body

```json
{
  "server_name": "testingwebs23423.riverbase.org",
  "theme_link": "",
  "env": {
    "VITE_VARIABLE_BACKEND": "http://localhost:8000",
    "VITE_VARIABLE_ID_STORE": "65677c4220d608fcf6eff114"
  },
  "files": [
    {
      "filename": "theme.json",
      "path": "themes",
      "data": {
        "color": {
          "primary": "#3674B7"
        }
      }
    }
  ]
}
```

| Variable    | Data Type                                                                 |
| ----------- | ------------------------------------------------------------------------- |
| server_name | String: eg. testingwebs23423.riverbase.org                                |
| theme_link  | String: eg. git@github.com:koompi/riverbase-themes.git                    |
| env         | Json Object Hashmap                                                       |
| files       | Json Object Array of filename, path and data of JSON Object without limit |

Response 200

```json
{
  "code": 200,
  "message": "Ok"
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

---

</details>

<details close="close">
<summary><b>DELETE</b> /hosting/delete/{server_name}</summary>

---

| Header      | Data Type              |
| ----------- | ---------------------- |
| server_name | String; eg: koompi.com |

Body

```json

```

| Variable | Data Type |
| -------- | --------- |
| None     | None      |

Response 200

```json
{
  "code": 200,
  "message": "Ok"
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

---

</details>
