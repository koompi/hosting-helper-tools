use super::fstools;

fn find_latest_log<S: AsRef<str>>(theme_path_absolute: S, write: bool) -> String {
    let mut i = 0;

    loop {
        let current_file = format!("{}/install.log-{}", theme_path_absolute.as_ref(), i);
        let log_path = std::path::Path::new(&current_file);
        match log_path.exists() {
            true => match write {
                true => {
                    i = i + 1;
                    continue;
                }
                false => break log_path.to_str().unwrap().to_string(),
            },
            false => match write {
                true => break log_path.to_str().unwrap().to_string(),
                false => break String::new(),
            },
        }
    }
}

pub fn read_log<S: AsRef<str>>(theme_path_absolute: S) -> (bool, bool, String) /* (finished, error, log) */
{
    let latest_log = find_latest_log(theme_path_absolute, false);
    match latest_log.is_empty() {
        true => return (true, true, String::from("Not Found!")),
        false => {
            let data = fstools::read_file(&latest_log);
            match data.is_empty() {
                true => return (false, false, String::from("Starting Up!")),
                false => {
                    let mut data_lines = data.trim().lines();
                    let (finished, error) = match data_lines.any(|line| line == "===") {
                        true => (true, data_lines.clone().last().unwrap() != "Ok"),
                        false => (false, false),
                    };
                    (
                        finished,
                        error,
                        data.lines()
                            .filter(|line| !(line == &"===" || line == &"Ok" || line == &"Err"))
                            .collect::<Vec<&str>>()
                            .join("\n"),
                    )
                }
            }
        }
    }
}

pub fn write_log<S: AsRef<str>>(
    action_log: S,
    theme_path_absolute: S,
    destination_file: Option<S>,
    finished: bool,
    error: bool,
) -> Option<String> {
    let destination_file = match destination_file {
        Some(destination_file) => destination_file.as_ref().to_string(),
        None => {
            let destination_file = find_latest_log(theme_path_absolute, true);
            std::fs::create_dir_all(
                destination_file
                    .split("/")
                    .filter_map(|each| (!each.contains(".log")).then(|| each))
                    .collect::<Vec<&str>>()
                    .join("/"),
            )
            .unwrap_or(());
            destination_file
        }
    };

    let text = match finished {
        true => match error {
            true => format!(
                "{}\t{}\n===\nError",
                chrono::Local::now(),
                action_log.as_ref()
            ),
            false => format!("{}\t{}\n===\nOk", chrono::Local::now(), action_log.as_ref()),
        },
        false => format!("{}\t{}\n", chrono::Local::now(), action_log.as_ref()),
    };

    fstools::write_file(destination_file.as_str(), text.as_str(), true).unwrap();
    Some(destination_file)
}
