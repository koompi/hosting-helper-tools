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

    let theme_path_absolute = match std::path::Path::new(&theme_path).exists() {
        true => Ok(theme_path),
        false => Err(ActixCustomResponse::new_text(
            400,
            format!("Server Name '{}' does not exists!", &args.get_server_name()),
        )),
    }?;

    let project_dir_handler = tokio::spawn(depl_fstools::git_clone(
        args.get_theme_link().to_string(),
        data.projroot.clone(),
        data.basepath.clone(),
        data.git_key.clone(),
    ));

    args.get_files().iter().for_each(|each| {
        let destination_file = match each.get_path() {
            Some(custom_path) => format!(
                "{}/{}/{}",
                theme_path_absolute,
                custom_path,
                each.get_filename()
            ),
            None => format!("{}/{}", theme_path_absolute, each.get_filename()),
        };
        fstools::write_file(
            destination_file.as_str(),
            &each.get_data().to_string(),
            false,
        )
        .unwrap()
    });

    let project_dir = match project_dir_handler.await.unwrap() {
        Ok(project_dir) => Ok(project_dir),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    let project_name = project_dir.split("/").last().unwrap().to_string();

    let copy_compose_file = tokio::spawn(tokio::fs::copy(
        format!("{}/{}/docker-compose.yaml", data.basepath, project_dir),
        format!("{}/docker-compose.yaml", theme_path_absolute),
    ));

    let copy_docker_file = tokio::spawn(tokio::fs::copy(
        format!("{}/{}/Dockerfile", data.basepath, project_dir),
        format!("{}/Dockerfile", theme_path_absolute),
    ));

    fstools::write_file(
        &format!("{}/.env", theme_path_absolute),
        &fstools::read_file(format!("{}/.env", theme_path_absolute))
            .trim()
            .split("\n")
            .map(|each| {
                let mut new_line = String::new();
                args.get_env().iter().for_each(|(k, v)| {
                    if each.starts_with("ROOTPROJ") {
                        new_line = format!("ROOTPROJ={}", project_dir);
                    } else if each.starts_with("ROOTPROJNAME") {
                        new_line = format!("ROOTPROJNAME={}", project_name);
                    } else if each.starts_with(k.as_str()) {
                        new_line = format!("{k}={v}");
                    } else {
                        new_line = each.to_string();
                    }
                });
                new_line
            })
            .collect::<Vec<String>>()
            .join("\n"),
        false,
    )
    .unwrap();

    match copy_compose_file.await.unwrap() {
        Ok(_) => Ok(()),
        Err(err) => Err(ActixCustomResponse::new_text(500, err.to_string())),
    }?;

    match copy_docker_file.await.unwrap() {
        Ok(_) => Ok(()),
        Err(err) => Err(ActixCustomResponse::new_text(500, err.to_string())),
    }?;

    match depl_fstools::stop_compose(&theme_path_absolute).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    match depl_fstools::compose_js(&theme_path_absolute).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    match depl_fstools::deploy_js(&theme_path_absolute).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::from("Updated"))))
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
        true => Err(ActixCustomResponse::new_text(
            400,
            format!("Server Name '{}' already existed!", &args.get_server_name()),
        )),
        false => Ok(()),
    }?;

    let available_port_handle = tokio::spawn(depl_fstools::scan_available_port());

    let project_dir_handler = tokio::spawn(depl_fstools::git_clone(
        args.get_theme_link().to_string(),
        data.projroot.clone(),
        data.basepath.clone(),
        data.git_key.clone(),
    ));

    let theme_path = format!("{}/{}", data.themepath, args.get_server_name());
    let theme_path_absolute = format!("{}/{}", data.basepath, theme_path);
    std::fs::create_dir_all::<&str>(theme_path_absolute.as_ref()).unwrap_or_default();

    args.get_files().iter().for_each(|each| {
        let destination_file = match each.get_path() {
            Some(custom_path) => {
                let custom_absolute_path = format!("{}/{}", theme_path_absolute, custom_path);
                std::fs::create_dir_all::<&str>(custom_absolute_path.as_ref()).unwrap_or_default();
                format!("{}/{}", custom_absolute_path, each.get_filename())
            }
            None => format!("{}/{}", theme_path_absolute, each.get_filename()),
        };
        std::fs::create_dir_all::<&str>(theme_path_absolute.as_ref()).unwrap_or_default();
        fstools::write_file(
            destination_file.as_str(),
            &each.get_data().to_string(),
            false,
        )
        .unwrap()
    });

    let mut env_map = args.get_env();

    env_map.insert(String::from("THEMEPATH"), theme_path);
    env_map.insert(String::from("BASEPATH"), data.basepath.clone());
    env_map.insert(
        String::from("CONTAINER_NAME"),
        args.get_server_name().to_string(),
    );

    let project_dir = match project_dir_handler.await.unwrap() {
        Ok(project_dir) => Ok(project_dir),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    let project_name = project_dir.split("/").last().unwrap().to_string();

    let copy_compose_file = tokio::spawn(tokio::fs::copy(
        format!("{}/{}/docker-compose.yaml", data.basepath, project_dir),
        format!("{}/docker-compose.yaml", theme_path_absolute),
    ));

    let copy_docker_file = tokio::spawn(tokio::fs::copy(
        format!("{}/{}/Dockerfile", data.basepath, project_dir),
        format!("{}/Dockerfile", theme_path_absolute),
    ));

    env_map.insert(String::from("ROOTPROJ"), project_dir);
    env_map.insert(String::from("ROOTPROJNAME"), project_name);

    let available_port = available_port_handle.await.unwrap();
    env_map.insert(String::from("PORT_NUMBER"), available_port.to_string());

    let env_data = env_map
        .iter()
        .map(|(each_key, each_value)| format!("{}={}", each_key, each_value))
        .collect::<Vec<String>>()
        .join("\n");

    fstools::write_file(&format!("{}/.env", theme_path_absolute), &env_data, false).unwrap();

    match copy_compose_file.await.unwrap() {
        Ok(_) => Ok(()),
        Err(err) => Err(ActixCustomResponse::new_text(500, err.to_string())),
    }?;

    match copy_docker_file.await.unwrap() {
        Ok(_) => Ok(()),
        Err(err) => Err(ActixCustomResponse::new_text(500, err.to_string())),
    }?;

    match depl_fstools::compose_js(&theme_path_absolute).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    match depl_fstools::deploy_js(&theme_path_absolute).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(
        200,
        format!("{}", available_port),
    )))
}

#[delete("/hosting/delete/{server_name}")]
pub async fn delete_hosting(
    req: HttpRequest,
    data: Data<EnvData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let server_name = match req.match_info().get("server_name") {
        Some(data) => Ok(data),
        None => Err(ActixCustomResponse::new_text(
            400,
            String::from("Missing Server Name"),
        )),
    }?;
    let theme_path = format!("{}/{}/{}", data.basepath, data.themepath, server_name);

    let theme_path_absolute = match std::path::Path::new(&theme_path).exists() {
        true => Ok(theme_path),
        false => Err(ActixCustomResponse::new_text(
            400,
            format!("Server Name '{}' does not exists!", server_name),
        )),
    }?;

    match depl_fstools::stop_compose(&theme_path_absolute).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
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
        None => Err(ActixCustomResponse::new_text(
            400,
            String::from("Missing Server Name"),
        )),
    }?;
    let theme_path = format!("{}/{}/{}", data.basepath, data.themepath, server_name);

    match std::path::Path::new(&theme_path).exists() {
        true => Ok(()),
        false => Err(ActixCustomResponse::new_text(
            400,
            format!("Server Name '{}' does not exists!", server_name),
        )),
    }?;

    let logs = match depl_fstools::log_compose(server_name).await {
        Ok(data) => Ok(data),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, logs)))
}

#[put("/hosting/update_git/{git_url}")]
pub async fn put_update_git_pull(
    req: HttpRequest,
    data: Data<EnvData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let url = match req.match_info().get("git_url") {
        Some(data) => Ok(data),
        None => Err(ActixCustomResponse::new_text(
            400,
            String::from("Missing git_url"),
        )),
    }?;
    let project_name = url.split("/").last().unwrap().replace(".git", "");
    let project_dir = format!("{}/{}/{}", data.basepath, data.projroot, project_name);
    depl_fstools::git_pull(&project_dir).await;

    let mut dir = tokio::fs::read_dir(format!("{}/{}", data.basepath, data.themepath))
        .await
        .unwrap();
    while let Some(entry) = dir.next_entry().await.unwrap_or_default() {
        let path = entry.path();
        let theme_path_absolute = path.to_str().unwrap();
        match fstools::read_file(format!("{}/.env", theme_path_absolute))
            .lines()
            .any(|each| each == format!("ROOTPROJNAME={}", project_name))
        {
            true => {
                let copy_compose_file = tokio::spawn(tokio::fs::copy(
                    format!("{}/{}/docker-compose.yaml", data.basepath, project_dir),
                    format!("{}/docker-compose.yaml", theme_path_absolute),
                ));

                let copy_docker_file = tokio::spawn(tokio::fs::copy(
                    format!("{}/{}/Dockerfile", data.basepath, project_dir),
                    format!("{}/Dockerfile", theme_path_absolute),
                ));

                match copy_compose_file.await.unwrap() {
                    Ok(_) => Ok(()),
                    Err(err) => Err(ActixCustomResponse::new_text(500, err.to_string())),
                }?;

                match copy_docker_file.await.unwrap() {
                    Ok(_) => Ok(()),
                    Err(err) => Err(ActixCustomResponse::new_text(500, err.to_string())),
                }?;

                match depl_fstools::stop_compose(&theme_path_absolute).await {
                    Ok(()) => Ok(()),
                    Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
                }?;

                match depl_fstools::compose_js(&theme_path_absolute).await {
                    Ok(()) => Ok(()),
                    Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
                }?;

                match depl_fstools::deploy_js(&theme_path_absolute).await {
                    Ok(()) => Ok(()),
                    Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
                }?;
            }
            false => continue,
        };
    }

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(200, String::new())))
}
