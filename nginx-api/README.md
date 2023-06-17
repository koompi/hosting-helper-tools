# NGINX-API

## How to run

- First, make file `settings.toml` from file `settings.toml.bk` at root directory of this project (login accounts need to be added to this file before running)

- Second, get binary from Compiling this project to the same place as `settings.toml`

- Next,
  ```bash
  sudo ./nginx-api
  ```

## Public API

<details close="close">
<summary><b>POST</b> /login</summary>

---

| Header | Data Type |
| ------ | --------- |
| None   | None      |

Body

```json
{
  "username": "isaac",
  "password": "123"
}
```

Response 200

```json
{
  "access_token": "eyJhbGciOiJFZERTQSJ9.eyJleHAiOjE2ODY5ODE5MTMsImlhdCI6MTY4Njk3ODMxMywidXNlcm5hbWUiOiJpc2FhYyIsInBhc3N3b3JkIjoiMTIzIn0.7wPA1dVRrBnRso4ut1R_N1Y_l66To8Z4s5nXLnZDfv8rKcITFP2hrjmhHw7v3XcareYFYHmifNo1McWDaKmMBQ",
  "refresh_token": "eyJhbGciOiJFZERTQSJ9.eyJleHAiOjE2ODcwNjQ3MTMsImlhdCI6MTY4Njk3ODMxMywidXNlcm5hbWUiOiJpc2FhYyIsInBhc3N3b3JkIjoiMTIzIn0.WLQyjUQrnrh573eRgzh67YRjtblQrJsJ4EDOYouvA4N7WAr_ZCJFBPGtivhrjmd6zE4OA3ZUCw28r3zml2MmCw"
}
```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 401   | Unautorised            |
| 500   | actual_error_goes_here |

---

</details>

## Protected API

All protected API endpoint needs:

| Header        | Data Type |
| ------------- | --------- |
| access_token  | String    |
| refresh_token | String    |

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
[
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

```json
{
  "server_name": "tellsela.com",
  "target_site": "http://koompi.com",
  "feature": "Proxy"
}
```

| Variable    | Data Type                                                                               |
| ----------- | --------------------------------------------------------------------------------------- |
| server_name | String: eg. rithy.org                                                                   |
| target_site | String: eg. https://weteka.org/user/rithy                                               |
| feature     | String: `Proxy` (forward without changing name) _or_ `Redirect` (forward changing name) |

Response 200

```json

```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

- Note:
  - `THIS API TAKE LONG TIME`
  - `server_name` must be first DNS pointed to this nginx server IP before add, otherwise it will error certificate generation
  - `server_name` must not include SCHEMA and must not already existed
  - `target_site` must be input in form of _SCHEMA://SUBDOMAIN.DOMAIN.TLD/WHATEVER_ otherwise it will error _BADREQUEST_
  - `feature` is ENUM on the backend

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

```

| Error | Body                   |
| ----- | ---------------------- |
| 400   | actual_error_goes_here |
| 500   | actual_error_goes_here |

- Note:
  - If `server_name` does not exist, it will error _BADREQUEST_

---

</details>
