use super::{super::init_migration, obj_response::ActixCustomResponse, HttpResponse};
use actix_web::{
    delete, get, post, put,
    web::{Json, Query},
    Error, HttpRequest,
};
use libnginx_wrapper::{
    dbtools::crud::select_all_from_tbl_nginxconf,
    http_server::{nginx_obj::NginxObj, remake_ssl, remove_nginx_conf, target_site::TargetSite},
};

#[derive(Debug, serde::Deserialize)]
pub struct AddNginxQueryString {
    cloudflare: bool,
}
impl AddNginxQueryString {
    fn get_cloudflare_bool(&self) -> &bool {
        &self.cloudflare
    }
}

#[get("/nginx/list")]
pub async fn get_nginx_list() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_vec_obj(
        200,
        select_all_from_tbl_nginxconf(),
    )))
}

#[post("/nginx/add")]
pub async fn post_add_nginx(
    args: Json<NginxObj>,
    qstring: Option<Query<AddNginxQueryString>>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let args = args.into_inner();

    match args.verify() {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomResponse::new_text(error_code, message)),
    }?;

    match args
        .setup_cloudflare(match qstring {
            Some(opt) => *opt.get_cloudflare_bool(),
            None => true,
        })
        .await
    {
        Ok(()) => Ok(()),
        Err((error_code, message)) => Err(ActixCustomResponse::new_text(error_code, message)),
    }?;

    match args.finish().await {
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
