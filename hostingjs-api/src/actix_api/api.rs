use super::{obj_req::ThemesData, obj_response::ActixCustomResponse, HttpResponse};
use actix_web::{post, web::Json};
use libdeploy_wrapper::{dbtools as depl_dbools, fstools as depl_fstools};
use libnginx_wrapper::fstools;

#[post("/hosting/update_existing")]
pub async fn post_hosting_update_existing(
    args: Json<ThemesData>,
) -> Result<HttpResponse, ActixCustomResponse> {
    let args = args.into_inner();
    match depl_dbools::query_existence_from_tbl_deploydata(&args.get_server_name()) {
        true => Ok(()),
        false => Err(ActixCustomResponse::new_text(
            400,
            format!("Server Name '{}' does not exists!", &args.get_server_name()),
        )),
    }?;

    let theme_path = depl_dbools::select_themedata_from_tbl_deploydata(&args.get_server_name());

    args.get_files().iter().for_each(|each| {
        let destination_file = match each.get_path() {
            Some(custom_path) => format!("{}/{}/{}", theme_path, custom_path, each.get_filename()),
            None => format!("{}/{}", theme_path, each.get_filename()),
        };
        fstools::write_file(
            destination_file.as_str(),
            &each.get_data().to_string(),
            false,
        )
        .unwrap()
    });

    match depl_fstools::install_js_dep(theme_path.clone()).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    match depl_fstools::build_js(theme_path.as_str()).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    match depl_fstools::pm2_restart(theme_path.as_str(), &args.get_server_name()).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    Ok(HttpResponse::Ok().finish())
}

#[post("/hosting/add")]
pub async fn post_hosting_add(args: Json<ThemesData>) -> Result<HttpResponse, ActixCustomResponse> {
    let args = args.into_inner();

    match depl_dbools::query_existence_from_tbl_deploydata(&args.get_server_name()) {
        true => Err(ActixCustomResponse::new_text(
            400,
            format!("Server Name '{}' already existed!", &args.get_server_name()),
        )),
        false => Ok(()),
    }?;

    let available_port_handle = tokio::spawn(depl_fstools::scan_available_port());

    let theme_path =
        match depl_fstools::git_clone(args.get_theme_link(), &args.get_server_name()).await {
            Ok(theme_path) => Ok(theme_path),
            Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
        }?;

    args.get_files().iter().for_each(|each| {
        let destination_file = match each.get_path() {
            Some(custom_path) => format!("{}/{}/{}", theme_path, custom_path, each.get_filename()),
            None => format!("{}/{}", theme_path, each.get_filename()),
        };
        fstools::write_file(
            destination_file.as_str(),
            &each.get_data().to_string(),
            false,
        )
        .unwrap()
    });

    let install_js_dep_handle = tokio::spawn(depl_fstools::install_js_dep(theme_path.clone()));

    let available_port = available_port_handle.await.unwrap();

    let mut env_map = args.get_env();
    env_map.insert(String::from("VITE_PORT"), available_port.to_string());
    let env_data = env_map
        .iter()
        .map(|(each_key, each_value)| format!("{}={}", each_key, each_value))
        .collect::<Vec<String>>()
        .join("\n");

    fstools::write_file(&format!("{}/.env", theme_path), &env_data, false).unwrap();

    match install_js_dep_handle.await.unwrap() {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    match depl_fstools::build_js(theme_path.as_str()).await {
        Ok(()) => Ok(()),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    let process_id = match depl_fstools::pm2_run(&theme_path, args.get_server_name()).await {
        Ok(process_id) => Ok(process_id),
        Err((code, message)) => Err(ActixCustomResponse::new_text(code, message)),
    }?;

    depl_dbools::insert_tbl_deploydata(
        process_id,
        available_port,
        &theme_path,
        args.get_server_name(),
    );

    Ok(HttpResponse::Ok().json(ActixCustomResponse::new_text(
        200,
        format!("{}", available_port),
    )))
}
