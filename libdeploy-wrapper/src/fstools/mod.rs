use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use rand::Rng;
use std::{
    fs,
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

pub async fn install_js_dep<S: AsRef<str>>(theme_path: S) -> Result<(), (u16, String)> {
    match Command::new("pnpm")
        .current_dir(theme_path.as_ref())
        .arg("install")
        .arg("--force")
        .output()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err((500, err.to_string())),
    }
}

pub async fn build_js<S: AsRef<str>>(theme_path: S) -> Result<(), (u16, String)> {
    match Command::new("pnpm")
        .current_dir(theme_path.as_ref())
        .arg("run")
        .arg("build")
        .output()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err((500, err.to_string())),
    }
}

pub async fn pm2_run(theme_path: &str, server_name: &str) -> Result<u32, (u16, String)> {
    match Command::new("pm2")
        .current_dir(theme_path)
        .arg("start")
        .args(["--name", server_name])
        .arg("pnpm")
        .arg("--")
        .arg("start")
        .output()
        .await
    {
        Ok(_) => Ok(()),
        Err(err) => Err((500, err.to_string())),
    }?;

    let checkid = String::from_utf8(
        Command::new("pm2")
            .current_dir(theme_path)
            .arg("id")
            .arg(server_name)
            .output()
            .await
            .unwrap()
            .stdout,
    )
    .unwrap().trim().parse::<u32>().unwrap();

    Ok(checkid)
}

pub fn git_clone(url: &str, server_name: &str) -> Result<String, (u16, String)> {
    let theme_path = dotenv::var("THEME_BASE_PATH").unwrap() + "/" + server_name;
    let theme_path_obj = Path::new(&theme_path);

    theme_path_obj
        .exists()
        .then(|| Some(fs::remove_dir_all(&theme_path)));

    let mut builder = setup_git_builder();
    match builder.clone(url, theme_path_obj) {
        Ok(_) => Ok(theme_path),
        Err(e) => Err((500, e.to_string())),
    }
}

fn setup_git_builder() -> RepoBuilder<'static> {
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(
        |_url, username_from_url, _allowed_types| {
            Cred::ssh_key_from_memory(
                username_from_url.unwrap(),
                None,
                dotenv::var("THEME_GIT_KEY").unwrap().as_ref(),
                None,
            )
        },
        // {
        // Cred::ssh_key(
        //     username_from_url.unwrap(),
        //     // Some(Path::new(&format!("{}/.ssh/id_ed25519.pub", env::var("HOME").unwrap()))),
        //     None,
        //     Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
        //     None,
        // )
        // }
    );

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    builder
}
