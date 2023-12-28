pub mod dbtools;
pub mod fstools;

use tokio::process::Command;

pub async fn init_migration() -> Result<(), (u16, String)> {
    let docker_output = tokio::spawn(Command::new("docker").arg("-v").output());
    let docker_compose_output = tokio::spawn(Command::new("docker").arg("compose").arg("version").output());
    get_res(docker_output.await.unwrap(), "docker")?;
    get_res(docker_compose_output.await.unwrap(), "docker compose")?;
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
