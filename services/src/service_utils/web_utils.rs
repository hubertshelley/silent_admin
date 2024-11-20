use std::borrow::Cow;

use configs::CFG;
use dto::common::client::{ClientInfo, ClientNetInfo, UserAgentInfo};
use serde::{Deserialize, Serialize};
use silent::header::HeaderMap;
use user_agent_parser::UserAgentParser;

pub async fn get_client_info(header: HeaderMap) -> ClientInfo {
    // 改为 header 中获取

    let user_agent = header.get("user-agent").unwrap().to_str().unwrap();
    let ua = get_user_agent_info(user_agent);
    let ip = get_remote_ip(header);
    let net = get_city_by_ip(&ip).await.unwrap();
    ClientInfo { net, ua }
}

pub fn get_remote_ip(header: HeaderMap) -> String {
    let ip = match header.get("X-Forwarded-For") {
        Some(x) => {
            let mut ips = x.to_str().unwrap().split(',');
            ips.next().unwrap().trim().to_string()
        }
        None => match header.get("X-Real-IP") {
            Some(x) => x.to_str().unwrap().to_string(),
            None => "".to_string(),
        },
    };
    ip
}

pub fn get_user_agent_info(user_agent: &str) -> UserAgentInfo {
    let ua_parser = UserAgentParser::from_path(&CFG.system.user_agent_parser).unwrap();
    let product_v = ua_parser.parse_product(user_agent);
    let os_v = ua_parser.parse_os(user_agent);
    let device_v = ua_parser.parse_device(user_agent);
    let browser = product_v.name.unwrap_or(Cow::Borrowed("")).to_string()
        + " "
        + product_v
            .major
            .unwrap_or(Cow::Borrowed(""))
            .to_string()
            .as_str();
    let os = os_v.name.unwrap_or(Cow::Borrowed("")).to_string()
        + " "
        + os_v.major.unwrap_or(Cow::Borrowed("")).to_string().as_str();
    let device = device_v.name.unwrap_or(Cow::Borrowed("")).to_string();
    UserAgentInfo {
        browser: browser.trim().to_string(),
        os: os.trim().to_string(),
        device,
    }
}

#[derive(Serialize, Deserialize)]
struct NetInfo {
    pub continent: String,
    pub country: String,
    pub zipcode: String,
    pub owner: String,
    pub isp: String,
    pub adcode: String,
    pub prov: String,
    pub city: String,
    pub district: String,
}

#[derive(Serialize, Deserialize)]
struct NetInfoResponse {
    pub code: String,
    pub data: NetInfo,
    pub ip: String,
}

async fn get_city_by_ip(ip: &str) -> Result<ClientNetInfo, Box<dyn std::error::Error>> {
    let url = "https://qifu-api.baidubce.com/ip/local/geo/v1/district?ip=".to_string() + ip;
    let resp = reqwest::get(url.as_str())
        .await?
        .text_with_charset("utf-8")
        .await?;
    let res = serde_json::from_str::<NetInfoResponse>(resp.as_str())?;
    let location = format!("{}{}{}", res.data.prov, res.data.city, res.data.district);
    let net_work = res.data.isp;
    Ok(ClientNetInfo {
        ip: res.ip,
        location,
        net_work,
    })
}
