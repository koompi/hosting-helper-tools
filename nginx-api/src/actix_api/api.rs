use super::{
    super::init_migration,
    obj_response::{ActixCustomResponse, CustomDnsStruct},
    querystring::{AddNginxQueryString, ListNginxQueryString},
    HttpResponse,
};
use actix_web::{
    delete, get, post, put,
    web::{Json, Query},
    Error, HttpRequest,
};
use libnginx_wrapper::{
    dbtools::crud::{
        select_all_by_feature_from_tbl_nginxconf, select_all_from_tbl_nginxconf,
        select_one_from_tbl_nginxconf,
    },
    http_server::{nginx_obj::NginxObj, remake_ssl, remove_nginx_conf, target_site::TargetSite},
};

#[get("/nginx/list")]
pub async fn get_nginx_list(
    qstring: Query<ListNginxQueryString>,
) -> Result<HttpResponse, ActixCustomResponse> {
    match qstring.get_server_name() {
        Some(server_name) => {
            let possible_data = match qstring.get_feature() {
                Some(_) => select_one_from_tbl_nginxconf(server_name, qstring.get_feature().as_ref()),
                None => select_one_from_tbl_nginxconf(server_name, None),
            };
            match possible_data {
                Ok(data) => {
                    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_nginx_obj(200, data)))
                }
                Err(err) => Err(ActixCustomResponse::new_text(500, err.to_string())),
            }
        }
        None => Ok(HttpResponse::Ok().json(ActixCustomResponse::new_vec_obj(
            200,
            match qstring.get_feature() {
                Some(feature) => select_all_by_feature_from_tbl_nginxconf(feature.to_string().as_str()),
                None => select_all_from_tbl_nginxconf(),
            },
        ))),
    }
}

#[post("/nginx/add")]
pub async fn post_add_nginx(
    args: Json<NginxObj>,
    qstring: Query<AddNginxQueryString>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let args = args.into_inner();

    println!("Enom: {} is illegally called", qstring.get_enom_bool());

    match args.verify() {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomResponse::new_text(error_code, message)),
    }?;

    match args
        .setup_cloudflare(*qstring.get_cloudflare_bool(), *qstring.get_ipcheck_bool())
        .await
    {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomResponse::new_text(error_code, message)),
    }?;

    match args.finish(*qstring.get_ssl_bool()).await {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomResponse::new_text(error_code, message)),
    }?;

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::from("Ok"))))
}

#[put("/nginx/update/{server_name}")]
pub async fn put_update_target_site(
    req: HttpRequest,
    target_site: Json<TargetSite>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let server_name = req.match_info().get("server_name").unwrap();
    let target_site = target_site.into_inner();

    match NginxObj::update_target(server_name, target_site).await {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomResponse::new_text(error_code, message)),
    }?;

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::from("Ok"))))
}

#[post("/cert/force/{server_name}")]
pub async fn post_force_cert(req: HttpRequest) -> Result<HttpResponse, ActixCustomResponse> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => Err(ActixCustomResponse::new_text(
            400,
            String::from("Missing Server Name"),
        )),
    }?;

    match remake_ssl(server_name) {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomResponse::new_text(error_code, message)),
    }?;

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::from("Ok"))))
}

#[post("/migration/force")]
pub async fn post_force_migration() -> Result<HttpResponse, Error> {
    match init_migration(true) {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomResponse::new_text(error_code, message)),
    }?;
    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::from("Ok"))))
}

#[delete("/nginx/delete/{server_name}")]
pub async fn delete_remove_nginx(req: HttpRequest) -> Result<HttpResponse, ActixCustomResponse> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => Err(ActixCustomResponse::new_text(
            400,
            String::from("Missing Server Name"),
        )),
    }?;

    match remove_nginx_conf(server_name.as_ref()) {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomResponse::new_text(error_code, message)),
    }?;

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::from("Ok"))))
}

#[get("/nginx/dns/{server_name}")]
pub async fn get_dns(req: HttpRequest) -> Result<HttpResponse, ActixCustomResponse> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => Err(ActixCustomResponse::new_text(
            400,
            String::from("Missing Server Name"),
        )),
    }?;

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_dns_obj(
        200,
        CustomDnsStruct::new(
            String::from(server_name),
            String::from("A"),
            libcloudflare_wrapper::get_public_ip(&libcloudflare_wrapper::get_client(), None).await,
            false,
            String::from("Auto"),
        ),
    )))
}
