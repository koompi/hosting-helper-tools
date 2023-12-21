use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use rand::Rng;
use std::{
    net::{SocketAddr, TcpListener},
    path::Path,
};
use tokio::process::Command;

pub async fn scan_available_port() -> u16 {
    loop {
        let random_port = rand::thread_rng().gen_range(1024..65535) as u16;
        match TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], random_port))) {
            Ok(_) => break random_port,
            Err(_) => continue,
        }
    }
}

pub async fn git_clone(
    url: String,
    project_dir: String,
    basepath: String,
    git_key: String,
) -> Result<String, (u16, String)> {
    let project_name = url.split("/").last().unwrap().replace(".git", "");
    let project_dir = match Path::new(&project_dir).is_absolute() {
        true => format!("{}/", project_dir),
        false => format!(
            "{}/{project_dir}/",
            std::env::current_exe()
                .unwrap()
                .parent()
                .unwrap()
                .to_str()
                .unwrap()
        ),
    };

    let theme_path_obj = Path::new::<str>(project_dir.as_ref());

    if !&theme_path_obj.exists() {
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key_from_memory(username_from_url.unwrap(), None, git_key.as_str(), None)
        });

        let mut fo = FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Prepare builder.
        let mut builder = RepoBuilder::new();
        builder.fetch_options(fo);

        match builder.clone(&url, theme_path_obj) {
            Ok(repo) => {
                return Ok(repo
                    .path()
                    .to_str()
                    .unwrap()
                    .replace(format!("{basepath}/").as_str(), ""))
            }
            Err(e) => Err((500, e.to_string())),
        }?;
    }

    Ok(format!("{}/{}", project_dir, project_name))
}

pub async fn compose_js<S: AsRef<str>>(theme_path: S) -> Result<(), (u16, String)> {
    match Command::new("docker")
        .current_dir(theme_path.as_ref())
        .arg("compose")
        .arg("-f")
        .arg(format!("{}/docker-compose.yaml", theme_path.as_ref()))
        .arg("build")
        .output()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err((500, err.to_string())),
    }
}



pub async fn stop_compose<S: AsRef<str>>(theme_path: S) -> Result<(), (u16, String)> {
    match Command::new("docker")
        .current_dir(theme_path.as_ref())
        .arg("compose")
        .arg("-f")
        .arg(format!("{}/docker-compose.yaml", theme_path.as_ref()))
        .arg("down")
        .output()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err((500, err.to_string())),
    }
}

pub async fn deploy_js<S: AsRef<str>>(theme_path: S) -> Result<(), (u16, String)> {
    match Command::new("docker")
        .current_dir(theme_path.as_ref())
        .arg("compose")
        .arg("-f")
        .arg(format!("{}/docker-compose.yaml", theme_path.as_ref()))
        .arg("up")
        .arg("-d")
        .output()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err((500, err.to_string())),
    }
}

// pub async fn install_js_dep<S: AsRef<str>>(theme_path: S) -> Result<(), (u16, String)> {
//     match Command::new("su")
//         .current_dir(theme_path.as_ref())
//         .arg("-c")
//         .arg("pnpm install --force")
//         .arg(dotenv::var("THEME_LOCAL_USER").unwrap())
//         .output()
//         .await
//     {
//         Ok(_) => Ok(()),
//         Err(err) => Err((500, err.to_string())),
//     }
// }

// pub async fn build_js<S: AsRef<str>>(theme_path: S) -> Result<(), (u16, String)> {
//     match Command::new("su")
//         .current_dir(theme_path.as_ref())
//         .arg("-c")
//         .arg("pnpm run build")
//         .arg(dotenv::var("THEME_LOCAL_USER").unwrap())
//         .output()
//         .await
//     {
//         Ok(_) => Ok(()),
//         Err(err) => Err((500, err.to_string())),
//     }
// }

// pub async fn pm2_restart(theme_path: &str, server_name: &str) -> Result<(), (u16, String)> {
//     match Command::new("su")
//         .current_dir(theme_path)
//         .arg("-c")
//         .arg(format!("pm2 restart {}", server_name))
//         .arg(dotenv::var("THEME_LOCAL_USER").unwrap())
//         .output()
//         .await
//     {
//         Ok(_) => Ok(()),
//         Err(err) => Err((500, err.to_string())),
//     }
// }

// pub async fn pm2_run(theme_path: &str, server_name: &str) -> Result<u32, (u16, String)> {
//     match Command::new("su")
//         .current_dir(theme_path)
//         .arg("-c")
//         .arg(format!("pm2 start --name '{}' pnpm -- start", server_name))
//         .arg(dotenv::var("THEME_LOCAL_USER").unwrap())
//         .output()
//         .await
//     {
//         Ok(_) => Ok(()),
//         Err(err) => Err((500, err.to_string())),
//     }?;

//     match Command::new("su")
//         .current_dir(theme_path)
//         .arg("-c")
//         .arg("pm2 save")
//         .arg(dotenv::var("THEME_LOCAL_USER").unwrap())
//         .output()
//         .await
//     {
//         Ok(_) => Ok(()),
//         Err(err) => Err((500, err.to_string())),
//     }?;

//     // let checkid = String::from_utf8(
//     //     Command::new("pm2")
//     //         .current_dir(theme_path)
//     //         .arg("id")
//     //         .arg(server_name)
//     //         .output()
//     //         .await
//     //         .unwrap()
//     //         .stdout,
//     // )
//     // .unwrap()
//     // .replace(&['[',']'], "")
//     // .trim()
//     // .parse::<u32>()
//     // .unwrap();

//     let checkid = 0;

//     Ok(checkid)
// }
