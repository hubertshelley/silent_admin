use std::time::Duration;

use futures::stream::{self};
use silent::prelude::{sse_reply, SSEEvent};
use silent::{Request, Response, Result};
use tokio_stream::StreamExt as _;

use dto::common::captcha::CaptchaImage;
use dto::system::server_info::SysInfo;
use services::system;

pub async fn get_captcha(_req: Request) -> Result<CaptchaImage> {
    let res = system::common::get_captcha();
    Ok(res)
}

pub async fn get_server_info(_req: Request) -> Result<SysInfo> {
    let res = system::server_info::get_oper_sys_info();

    Ok(res)
}

//  这个不知道为啥有问题
pub async fn get_server_info_sse(_req: Request) -> Result<Response> {
    let stream = stream::repeat_with(|| {
        let sys_info = system::server_info::get_oper_sys_info();
        SSEEvent::default()
            .data(serde_json::to_string(&sys_info).unwrap_or_else(|_| "0".to_string()))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    sse_reply(stream)
}
