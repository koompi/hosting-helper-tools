use super::super::obj_req::ThemesData;
use super::fstools;
use super::log_action::write_log;
use libdeploy_wrapper::fstools as depl_fstools;

pub async fn default_action(args: ThemesData, data: crate::EnvData, is_add: bool) {
    let mut can_continue = true;
    let theme_path = format!("{}/{}", data.themepath, args.get_server_name());
    let theme_path_absolute = format!("{}/{}", data.basepath, theme_path);

    let log_file = write_log(
        format!("Cloning Project").as_str(),
        theme_path_absolute.as_ref(),
        None,
        false,
        false,
    )
    .unwrap();

    let project_dir_handler = tokio::spawn(depl_fstools::git_clone(
        args.get_theme_link().to_string(),
        data.projroot.clone(),
        data.basepath.clone(),
        data.git_key.clone(),
    ));

    write_log(
        "Initiate Workplace",
        theme_path_absolute.as_str(),
        Some(log_file.as_str()),
        false,
        false,
    )
    .unwrap();

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

        write_log(
            format!("Writing {destination_file}").as_str(),
            theme_path_absolute.as_str(),
            Some(log_file.as_str()),
            false,
            false,
        )
        .unwrap();

        std::fs::create_dir_all::<&str>(theme_path_absolute.as_ref()).unwrap_or_default();
        fstools::write_file(
            destination_file.as_str(),
            &each.get_data().to_string(),
            false,
        )
        .unwrap()
    });

    let (step_message, project_dir, error) = match project_dir_handler.await.unwrap() {
        Ok(project_dir) => (String::from("Cloned Project Complete"), project_dir, false),
        Err((code, message)) => {
            can_continue = false;
            (format!("Error {code}: {message}"), String::new(), true)
        }
    };
    write_log(
        step_message.as_str(),
        theme_path_absolute.as_str(),
        Some(log_file.as_str()),
        error,
        error,
    )
    .unwrap();

    if can_continue {
        let project_name = project_dir.split("/").last().unwrap().to_string();

        write_log(
            "Set Up Hosting Environment",
            theme_path_absolute.as_str(),
            Some(log_file.as_str()),
            false,
            false,
        )
        .unwrap();

        let copy_compose_file = tokio::spawn(tokio::fs::copy(
            format!("{}/{}/docker-compose.yaml", data.basepath, project_dir),
            format!("{}/docker-compose.yaml", theme_path_absolute),
        ));

        let copy_docker_file = tokio::spawn(tokio::fs::copy(
            format!("{}/{}/Dockerfile", data.basepath, project_dir),
            format!("{}/Dockerfile", theme_path_absolute),
        ));

        match is_add {
            true => {
                let available_port_handle = tokio::spawn(depl_fstools::scan_available_port());

                let mut env_map = args.get_env();

                env_map.insert(String::from("THEMEPATH"), theme_path);
                env_map.insert(String::from("BASEPATH"), data.basepath.clone());
                env_map.insert(
                    String::from("CONTAINER_NAME"),
                    args.get_server_name().to_string(),
                );

                env_map.insert(String::from("ROOTPROJ"), project_dir);
                env_map.insert(String::from("ROOTPROJNAME"), project_name);

                let available_port = available_port_handle.await.unwrap();
                env_map.insert(String::from("PORT_NUMBER"), available_port.to_string());
                let env_data = env_map
                    .iter()
                    .map(|(each_key, each_value)| format!("{}={}", each_key, each_value))
                    .collect::<Vec<String>>()
                    .join("\n");

                fstools::write_file(&format!("{}/.env", theme_path_absolute), &env_data, false)
                    .unwrap();
            }
            false => {
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
            }
        }

        let (step_message, error) = match copy_compose_file.await.unwrap() {
            Ok(_) => (format!("Copied Build File 01"), false),
            Err(err) => {
                can_continue = false;
                (format!("Error 500: {err}"), true)
            }
        };
        write_log(
            step_message.as_str(),
            theme_path_absolute.as_str(),
            Some(log_file.as_str()),
            error,
            error,
        )
        .unwrap();

        if can_continue {
            let (step_message, error) = match copy_docker_file.await.unwrap() {
                Ok(_) => (format!("Copied Build File 02"), false),
                Err(err) => {
                    can_continue = false;
                    (format!("Error 500: {err}"), true)
                }
            };
            write_log(
                step_message.as_str(),
                theme_path_absolute.as_str(),
                Some(log_file.as_str()),
                error,
                error,
            )
            .unwrap();
        }

        if can_continue && is_add == false {
            let (step_message, error) = match depl_fstools::stop_compose(&theme_path_absolute).await
            {
                Ok(_) => (format!("Stopped the Server"), false),
                Err((code, message)) => {
                    can_continue = false;
                    (format!("Error {code}: {message}"), true)
                }
            };

            write_log(
                step_message.as_str(),
                theme_path_absolute.as_str(),
                Some(log_file.as_str()),
                error,
                error,
            )
            .unwrap();
        }

        if can_continue {
            let (step_message, error) = match depl_fstools::compose_js(&theme_path_absolute).await {
                Ok(_) => (format!("Installed and Built the Website"), false),
                Err((code, message)) => {
                    can_continue = false;
                    (format!("Error {code}: {message}"), true)
                }
            };

            write_log(
                step_message.as_str(),
                theme_path_absolute.as_str(),
                Some(log_file.as_str()),
                error,
                error,
            )
            .unwrap();

            if can_continue {
                let step_message = match depl_fstools::deploy_js(&theme_path_absolute).await {
                    Ok(_) => format!("Deployed the Server"),
                    Err((code, message)) => {
                        can_continue = false;
                        format!("Error {code}: {message}")
                    }
                };

                write_log(
                    step_message.as_str(),
                    theme_path_absolute.as_str(),
                    Some(log_file.as_str()),
                    true,
                    !can_continue,
                )
                .unwrap();
            }
        }
    }
}
