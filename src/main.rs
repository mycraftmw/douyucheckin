use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;

use anyhow::Result;
use chrono::Local;
use tokio::time::{delay_for, Duration};

async fn check_status(room_id: i32) -> Result<bool> {
    let req_url = format!("http://open.douyucdn.cn/api/RoomApi/room/{}", room_id);
    let res = reqwest::get(&req_url).await?.text().await?;
    let path = Path::new("douyu_info");
    let mut file = match File::create(&path) {
        Err(_) => panic!("Cannot create tmp file"),
        Ok(file) => file,
    };
    match file.write_all(res.as_bytes()) {
        Err(_) => panic!("Cannot write file"),
        Ok(_) => (),
    }
    let status = Command::new("jq")
        .arg(".data.room_status")
        .arg(path.to_str().unwrap())
        .output()
        .expect("failed to parse json");
    let output = String::from_utf8_lossy(&status.stdout)
        .chars()
        .nth(1)
        .unwrap();
    if output != '2' {
        Ok(true)
    } else {
        Ok(false)
    }
}

async fn send_message() -> Result<()> {
    let sc_key = "";
    let sc_url = format!("https://sc.ftqq.com/{}.send?text=老鸡头开播了！", sc_key);
    reqwest::get(&sc_url).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let room_id = 237974;
    loop {
        let status = check_status(room_id).await?;
        if status {
            send_message().await?;
            break;
        }
        println!("都{}了！还没开播！", Local::now());
        delay_for(Duration::from_secs(10)).await;
    }
    Ok(())
}
