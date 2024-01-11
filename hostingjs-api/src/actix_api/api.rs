use crate::EnvData;

use super::{obj_req::ThemesData, obj_response::ActixCustomResponse, HttpResponse};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json},
    HttpRequest,
};
use libdeploy_wrapper::fstools as depl_fstools;
use libnginx_wrapper::fstools;

#[put("/hosting/update")]
pub async fn put_hosting_update(
    args: Json<ThemesData>,
    data: Data<crate::EnvData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let args = args.into_inner();

    let theme_path = format!(
        "{}/{}/{}",
        data.basepath,
        data.themepath,
        args.get_server_name()
    );

    let _ = match std::path::Path::new(&theme_path).exists() {
        true => Ok(theme_path),
        false => {
            return Err(ActixCustomResponse::new_text(
                400,
                format!("Server Name '{}' does not exists!", &args.get_server_name()),
            ))
        }
    }?;

    tokio::spawn(super::actions::hosting::default_action(
        args,
        data.into_inner().as_ref().clone(),
        false,
    ));

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::from("Pending"))))
}

#[post("/hosting/add")]
pub async fn post_hosting_add(
    args: Json<ThemesData>,
    data: Data<crate::EnvData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let args = args.into_inner();

    let theme_path = format!(
        "{}/{}/{}",
        data.basepath,
        data.themepath,
        args.get_server_name()
    );

    match std::path::Path::new(&theme_path).exists() {
        true => {
            return Err(ActixCustomResponse::new_text(
                400,
                format!("Server Name '{}' already existed!", &args.get_server_name()),
            ))
        }
        false => Ok(()),
    }?;

    tokio::spawn(super::actions::hosting::default_action(
        args,
        data.into_inner().as_ref().clone(),
        true,
    ));

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::from("Pending"))))
}

#[get("/hosting/port/{server_name}")]
pub async fn get_server_port(
    req: HttpRequest,
    data: Data<crate::EnvData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => {
            return Err(ActixCustomResponse::new_text(
                400,
                String::from("Missing Server Name"),
            ))
        }
    }?;

    let theme_path = format!("{}/{}/{}", data.basepath, data.themepath, server_name);

    let theme_path = match std::path::Path::new(&theme_path).exists() {
        true => Ok(theme_path),
        false => {
            return Err(ActixCustomResponse::new_text(
                400,
                format!("Server Name '{}' does not exists!", server_name),
            ));
        }
    }?;

    let port: u16 = fstools::read_file(format!("{}/.env", theme_path))
        .lines()
        .filter_map(|each| each.contains("PORT_NUMBER").then(|| each))
        .collect::<Vec<&str>>()
        .join("")
        .split("=")
        .last()
        .unwrap()
        .parse::<u16>()
        .unwrap();

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, format!("{port}"))))
}

#[delete("/hosting/delete/{server_name}")]
pub async fn delete_hosting(
    req: HttpRequest,
    data: Data<EnvData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => {
            return Err(ActixCustomResponse::new_text(
                400,
                String::from("Missing Server Name"),
            ))
        }
    }?;

    let theme_path = format!("{}/{}/{}", data.basepath, data.themepath, server_name);

    let theme_path_absolute = match std::path::Path::new(&theme_path).exists() {
        true => Ok(theme_path),
        false => {
            return Err(ActixCustomResponse::new_text(
                400,
                format!("Server Name '{}' does not exists!", server_name),
            ))
        }
    }?;

    match depl_fstools::stop_compose(&theme_path_absolute).await {
        Ok(()) => Ok(()),
        Err((code, message)) => return Err(ActixCustomResponse::new_text(code, message)),
    }?;

    tokio::fs::remove_dir_all(theme_path_absolute)
        .await
        .unwrap_or(());

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::from("Ok"))))
}

#[get("/hosting/get_log/{server_name}")]
pub async fn get_hosting_log(
    req: HttpRequest,
    data: Data<EnvData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => {
            return Err(ActixCustomResponse::new_text(
                400,
                String::from("Missing Server Name"),
            ))
        }
    }?;

    let theme_path = format!("{}/{}/{}", data.basepath, data.themepath, server_name);

    match std::path::Path::new(&theme_path).exists() {
        true => Ok(()),
        false => {
            return Err(ActixCustomResponse::new_text(
                400,
                format!("Server Name '{}' does not exists!", server_name),
            ))
        }
    }?;

    let (finished, error, logs) = super::actions::log_action::read_log(theme_path);
    match finished {
        true => match error {
            false => {
                let docker_logs = match depl_fstools::log_compose(server_name).await {
                    Ok(data) => Ok(data),
                    Err((code, message)) => {
                        return Err(ActixCustomResponse::new_text(code, message))
                    }
                }?;
                Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(
                    200,
                    format!("{}/n{}", logs, docker_logs),
                )))
            }
            true => return Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(500, logs))),
        },
        false => Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(202, logs))),
    }
}

#[put("/hosting/update_git/{git_url}")]
pub async fn put_update_git_pull(
    req: HttpRequest,
    data: Data<EnvData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let url = match req.match_info().get("git_url") {
        Some(data) => Ok(data),
        None => {
            return Err(ActixCustomResponse::new_text(
                400,
                String::from("Missing git_url"),
            ))
        }
    }?;

    tokio::spawn(super::actions::git_update::default_action(
        data.as_ref().to_owned(),
        url.to_owned(),
    ));

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::new())))
}

#[get("/hosting/update_git/{git_url}/log")]
pub async fn get_log_update_git_pull(
    req: HttpRequest,
    data: Data<EnvData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let url = match req.match_info().get("git_url") {
        Some(data) => Ok(data),
        None => {
            return Err(ActixCustomResponse::new_text(
                400,
                String::from("Missing git_url"),
            ));
        }
    }?;

    tokio::spawn(super::actions::git_update::default_action(
        data.as_ref().to_owned(),
        url.to_owned(),
    ));

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::new())))
}
