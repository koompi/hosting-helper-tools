use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks};
use rand::Rng;
use std::{env, fs, path::Path, net::{TcpListener, SocketAddr}};
use tokio::process::Command;

pub async fn scan_available_port() -> u16 {
    loop {
        let random_port = rand::thread_rng().gen_range(1024..65535) as u16;
        match TcpListener::bind(SocketAddr::from(([127,0,0,1], random_port))) {
            Ok(_) => break random_port,
            Err(_) => continue
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

pub async fn spawn_child(theme_path: &str) -> Result<u32, (u16, String)> {
    let child_command = Command::new("pnpm")
        .current_dir(theme_path)
        .arg("run")
        .arg("start")
        .spawn();
    let mut child_id = 0u32;
    match child_command {
        Ok(mut child) => Ok({
            child_id = child.id().unwrap();
            let _ = child.wait().await;
        }),
        Err(e) => Err((500, e.to_string())),
    }?;
    Ok(child_id)
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
    callbacks.credentials(|_url, username_from_url, _allowed_types| {
        Cred::ssh_key(
            username_from_url.unwrap(),
            None,
            Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
            None,
        )
    });

    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks);

    // Prepare builder.
    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);

    builder
}
