use git2::{build::RepoBuilder, Cred, FetchOptions, RemoteCallbacks, Repository};
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

pub async fn git_pull<S: AsRef<str>>(project_dir: S) {
    let repo = Repository::open(project_dir.as_ref()).unwrap();

    repo.find_remote("origin").unwrap()
        .fetch(&["main"], None, None).unwrap();

    let fetch_head = repo.find_reference("FETCH_HEAD").unwrap();
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head).unwrap();
    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();
    if analysis.0.is_fast_forward() {
        let refname = format!("refs/heads/{}", "main");
        let mut reference = repo.find_reference(&refname).unwrap();
        reference.set_target(fetch_commit.id(), "Fast-Forward").unwrap();
        repo.set_head(&refname).unwrap();
        repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force())).unwrap();
    }
}

pub async fn git_clone(
    url: String,
    project_dir: String,
    basepath: String,
    git_key: String,
) -> Result<String, (u16, String)> {
    let project_name = url.split("/").last().unwrap().replace(".git", "");

    let git_path = format!("{}/{}/{}", basepath, project_dir, project_name);
    let theme_path_obj = Path::new(&git_path);

    if !&theme_path_obj.exists() {
        tokio::fs::create_dir_all(&git_path).await.unwrap();
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
                let repo_path = repo
                    .path()
                    .to_str()
                    .unwrap()
                    .replace(format!("{basepath}/").as_str(), "")
                    .replace("/.git", "");

                let repo_path = match repo_path.strip_suffix("/") {
                    Some(s) => s,
                    None => &repo_path,
                }
                .to_string();

                return Ok(repo_path);
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
        Ok(res) => match res.status.success() {
            true => Ok(()),
            false => Err((500, String::from_utf8(res.stderr).unwrap())),
        },
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
        Ok(res) => match res.status.success() {
            true => Ok(()),
            false => Err((500, String::from_utf8(res.stderr).unwrap())),
        },
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
        Ok(res) => match res.status.success() {
            true => Ok(()),
            false => Err((500, String::from_utf8(res.stderr).unwrap())),
        },
        Err(err) => Err((500, err.to_string())),
    }
}

pub async fn log_compose<S: AsRef<str>>(server_name: S) -> Result<String, (u16, String)> {
    match tokio::process::Command::new("docker")
        .arg("logs")
        .arg("-t")
        .arg(server_name.as_ref())
        .output()
        .await
    {
        Ok(res) => match res.status.success() {
            true => Ok(String::from_utf8(res.stdout).unwrap()),
            false => Err((500, String::from_utf8(res.stderr).unwrap())),
        },
        Err(err) => Err((500, err.to_string())),
    }
}
