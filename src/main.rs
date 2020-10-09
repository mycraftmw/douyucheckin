use std::collections::HashMap;

use anyhow::Result;
use chrono::Local;
use serde::{Deserialize, Serialize};
use tokio::time::{delay_for, Duration};

#[tokio::main]
async fn main() -> Result<()> {
    // fill in the room number
    let room_id = 0;
    // fill in the cookie string
    let raw_cookie = "";
    let cookies = raw_cookie
        .split_terminator(";")
        .map(|e| {
            let t = e.trim().splitn(2, '=').collect::<Vec<_>>();
            (t[0], t[1])
        })
        .collect::<HashMap<_, _>>();
    let token = format!(
        "{}_{}_{}_{}_{}",
        cookies.get("acf_uid").unwrap(),
        cookies.get("acf_biz").unwrap(),
        cookies.get("acf_stk").unwrap(),
        cookies.get("acf_ct").unwrap(),
        cookies.get("acf_ltkid").unwrap()
    );
    loop {
        let status = get_room_stats(room_id).await?;
        if status.data.room_status != "2" {
            if check_in(raw_cookie, &token, room_id).await? {
                println!("{} | {} 签到成功！", status.data.owner_name, room_id);
            } else {
                println!("{} | {} 签到失败！！！", status.data.owner_name, room_id);
            }
            break;
        }
        println!("都{}了！还没开播！", Local::now());
        delay_for(Duration::from_secs(3)).await;
    }
    Ok(())
}

async fn check_in(raw_cookie: &str, token: &String, room_id: i32) -> Result<bool> {
    let ck_url = format!(
        "https://apiv2.douyucdn.cn/japi/roomuserlevel/apinc/checkIn?rid={}",
        room_id
    );
    let client = reqwest::Client::new();
    let res = client
        .post(&ck_url)
        .header("Cookie", raw_cookie)
        .header("token", token)
        .send()
        .await?;
    Ok(res.status().is_success())
}

#[derive(Serialize, Deserialize, Debug)]
struct RoomStats {
    error: i32,
    data: RoomStatsData,
}
#[derive(Serialize, Deserialize, Debug)]
struct RoomStatsData {
    room_status: String,
    owner_name: String,
}

async fn get_room_stats(room_id: i32) -> Result<RoomStats> {
    let req_url = format!("http://open.douyucdn.cn/api/RoomApi/room/{}", room_id);
    let res = reqwest::get(&req_url).await?.text().await?;
    Ok(serde_json::from_str(&res).unwrap())
}
