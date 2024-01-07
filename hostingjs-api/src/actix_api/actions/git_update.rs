use super::fstools;
use super::log_action::write_log;
use libdeploy_wrapper::fstools as depl_fstools;

pub async fn default_action<S: AsRef<str>>(data: crate::EnvData, url: S) {
    let project_name = url.as_ref().split("/").last().unwrap().replace(".git", "");
    let log_dir = format!(
        "{}/{}/git_update_log/{}",
        data.basepath, data.projroot, project_name
    );
    std::fs::create_dir_all(&log_dir).unwrap_or(());
    let log_path = write_log("Set Up Environment", &log_dir, None, false, false).unwrap();

    let project_dir = format!("{}/{}/{}", data.basepath, data.projroot, project_name);
    write_log("Commence Pulling", &log_dir, Some(&log_path), false, false);
    depl_fstools::git_pull(&project_dir).await;

    write_log(
        "Commence Rebuild Projects",
        &log_dir,
        Some(&log_path),
        false,
        false,
    );
    let mut dir = tokio::fs::read_dir(format!("{}/{}", data.basepath, data.themepath))
        .await
        .unwrap();

    while let Some(entry) = dir.next_entry().await.unwrap_or_default() {
        let path = entry.path();
        let theme_path_absolute = path.to_str().unwrap();

        write_log(
            format!(
                "Updating {}",
                theme_path_absolute.split("/").last().unwrap()
            )
            .as_str(),
            log_dir.as_ref(),
            Some(log_path.as_str()),
            false,
            false,
        );
        match fstools::read_file(format!("{}/.env", theme_path_absolute))
            .lines()
            .any(|each| each == format!("ROOTPROJNAME={}", project_name))
        {
            true => {
                let mut can_continue = true;

                let copy_compose_file = tokio::spawn(tokio::fs::copy(
                    format!("{}/{}/docker-compose.yaml", data.basepath, project_dir),
                    format!("{}/docker-compose.yaml", theme_path_absolute),
                ));

                let copy_docker_file = tokio::spawn(tokio::fs::copy(
                    format!("{}/{}/Dockerfile", data.basepath, project_dir),
                    format!("{}/Dockerfile", theme_path_absolute),
                ));

                let (step_message, error) = match copy_compose_file.await.unwrap() {
                    Ok(_) => (format!("Copied Build File 01"), false),
                    Err(err) => {
                        can_continue = false;
                        (format!("Error 500: {err}"), true)
                    }
                };
                write_log(
                    step_message.as_str(),
                    theme_path_absolute.as_ref(),
                    Some(log_path.as_str()),
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
                        theme_path_absolute.as_ref(),
                        Some(log_path.as_str()),
                        error,
                        error,
                    )
                    .unwrap();
                }

                if can_continue {
                    let (step_message, error) =
                        match depl_fstools::stop_compose(&theme_path_absolute).await {
                            Ok(_) => (format!("Stopped the Server"), false),
                            Err((code, message)) => {
                                can_continue = false;
                                (format!("Error {code}: {message}"), true)
                            }
                        };

                    write_log(
                        step_message.as_str(),
                        theme_path_absolute.as_ref(),
                        Some(log_path.as_str()),
                        error,
                        error,
                    )
                    .unwrap();
                }

                if can_continue {
                    let (step_message, error) =
                        match depl_fstools::compose_js(&theme_path_absolute).await {
                            Ok(_) => (format!("Installed and Built the Website"), false),
                            Err((code, message)) => {
                                can_continue = false;
                                (format!("Error {code}: {message}"), true)
                            }
                        };

                    write_log(
                        step_message.as_str(),
                        theme_path_absolute.as_ref(),
                        Some(log_path.as_str()),
                        error,
                        error,
                    )
                    .unwrap();

                    if can_continue {
                        let step_message = match depl_fstools::deploy_js(&theme_path_absolute).await
                        {
                            Ok(_) => format!("Deployed the Server"),
                            Err((code, message)) => {
                                can_continue = false;
                                format!("Error {code}: {message}")
                            }
                        };

                        write_log(
                            step_message.as_str(),
                            theme_path_absolute.as_ref(),
                            Some(log_path.as_str()),
                            true,
                            !can_continue,
                        )
                        .unwrap();
                    }
                }
            }
            false => continue,
        };
    }
}
