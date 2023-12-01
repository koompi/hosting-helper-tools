pub mod dbtools;
pub mod fstools;

use tokio::process::Command;

pub async fn init_migration(force: bool) -> Result<(), (u16, String)> {
    let npm_output = tokio::spawn(Command::new("npm").arg("-v").output());
    let pnpm_output = tokio::spawn(Command::new("pnpm").arg("-v").output());
    let node_output = tokio::spawn(Command::new("node").arg("-v").output());
    let pm2_output = tokio::spawn(Command::new("pm2").arg("-v").arg("-s").output());
    get_res(node_output.await.unwrap(), "node")?;
    get_res(npm_output.await.unwrap(), "npm")?;
    get_res(pnpm_output.await.unwrap(), "pnpm")?;
    get_res(pm2_output.await.unwrap(), "pm2")?;

    Command::new("pm2").arg("startup").output().await.unwrap();

    let _ = libdatabase::db_migration(
        libdatabase::DBClient::LibDeploy,
        match force {
            true => Some(libdatabase::DBClient::LibDeploy),
            false => None,
        },
    )
    .unwrap_or_else(|| 0);
    Ok(())
}

fn get_res(
    cmd_output: Result<std::process::Output, std::io::Error>,
    prog: &str,
) -> Result<(), (u16, String)> {
    match cmd_output {
        Ok(output) => match output.status.success() {
            true => Ok(println!(
                "Your {prog} version is {}",
                String::from_utf8(output.stdout).unwrap().trim()
            )),
            false => Err((
                500,
                format!(
                    "Your {} has problem. Please reinstall or recheck if usable",
                    prog
                ),
            )),
        },
        Err(_) => Err((
            500,
            format!(
                "Your {} has problem. Please reinstall or recheck if usable",
                prog
            ),
        )),
    }
}
