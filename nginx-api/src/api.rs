use super::init_migration;
use actix_web::{delete, error, get, post, web::Json, Error, HttpRequest, HttpResponse};
use libnginx_wrapper::{
    dbtools::crud::select_all_from_tbl_nginxconf,
    http_server::{remake_ssl, remove_nginx_conf, NginxObj},
};

#[get("/nginx/list")]
pub async fn get_nginx_list() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(select_all_from_tbl_nginxconf()))
}

#[post("/nginx/add")]
pub async fn post_add_nginx(args: Json<NginxObj>) -> Result<HttpResponse, Error> {

    let args = args.into_inner();

    match args.verify() {
        Ok(()) => Ok(()),
        Err(err) => Err(error::ErrorBadRequest(Json(err))),
    }?;

    match args.finish() {
        Ok(()) => Ok(()),
        Err(err) => Err(error::ErrorInternalServerError(Json(err))),
    }?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/cert/force/{server_name}")]
pub async fn post_force_cert(req: HttpRequest) -> Result<HttpResponse, Error> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => Err(error::ErrorBadRequest("Missing Server Name")),
    }?;

    match remake_ssl(server_name) {
        Ok(()) => Ok(()),
        Err(err) => Err(error::ErrorInternalServerError(Json(err))),
    }?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/migration/force")]
pub async fn post_force_migration() -> Result<HttpResponse, Error> {
    init_migration(true);
    Ok(HttpResponse::Ok().finish())
}

#[delete("/nginx/delete/{server_name}")]
pub async fn delete_remove_nginx(req: HttpRequest) -> Result<HttpResponse, Error> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => Err(error::ErrorBadRequest("Missing Server Name")),
    }?;

    match remove_nginx_conf(server_name) {
        Ok(()) => Ok(()),
        Err(err) => Err(error::ErrorInternalServerError(Json(err))),
    }?;

    Ok(HttpResponse::Ok().finish())
}
