pub mod dbtools;
pub mod fstools;

use tokio::process::Command;

pub async fn init_migration() -> Result<(), (u16, String)> {
    let npm_output = tokio::spawn(Command::new("npm").arg("-v").output());
    let pnpm_output = tokio::spawn(Command::new("pnpm").arg("-v").output());
    let node_output = tokio::spawn(Command::new("node").arg("-v").output());
    let pm2_output = tokio::spawn(Command::new("pm2").arg("-v").output());

    match npm_output.await.unwrap() {
        Ok(output) => match output.status.success() {
            true => Ok(println!("Your npm version is {}", String::from_utf8(output.stdout).unwrap().trim())),
            false => Err((500, String::from("Your npm has problem. Please reinstall or recheck if usable"))),
        },
        Err(_) => Err((500, String::from("Your npm has problem. Please reinstall or recheck if usable")))
    }?;

    match node_output.await.unwrap() {
        Ok(output) => match output.status.success() {
            true => Ok(println!("Your node version is {}", String::from_utf8(output.stdout).unwrap().trim())),
            false => Err((500, String::from("Your node has problem. Please reinstall or recheck if usable"))),
        },
        Err(_) => Err((500, String::from("Your node has problem. Please reinstall or recheck if usable")))
    }?;

    match pnpm_output.await.unwrap() {
        Ok(output) => match output.status.success() {
            true => Ok(println!("Your pnpm version is {}", String::from_utf8(output.stdout).unwrap().trim())),
            false => Err((500, String::from("Your pnpm has problem. Please reinstall or recheck if usable"))),
        },
        Err(_) => Err((500, String::from("Your pnpm has problem. Please reinstall or recheck if usable")))
    }?;

    match pm2_output.await.unwrap() {
        Ok(output) => match output.status.success() {
            true => Ok(println!("Your pm2 version is {}", String::from_utf8(output.stdout).unwrap().trim())),
            false => Err((500, String::from("Your pm2 has problem. Please reinstall or recheck if usable"))),
        },
        Err(_) => Err((500, String::from("Your pm2 has problem. Please reinstall or recheck if usable")))
    }?;

    Ok(())
}