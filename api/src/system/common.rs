#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
use std::{convert::Infallible, time::Duration};

use app_service::system;
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
use axum::response::sse::{Event, Sse};
use db::common::{captcha::CaptchaImage, res::Res};
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
use db::system::models::server_info::SysInfo;
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
use futures::stream::{self, Stream};
use silent::{Request, Result};
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
use tokio_stream::StreamExt as _;

pub async fn get_captcha(_req: Request) -> Result<Res<CaptchaImage>> {
    let res = system::common::get_captcha();
    Ok(Res::with_data(res))
}

#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
pub async fn get_server_info() -> Res<SysInfo> {
    let res = system::server_info::get_oper_sys_info();

    Res::with_data(res)
}

//  这个不知道为啥有问题
#[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
pub async fn get_server_info_sse() -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let stream = stream::repeat_with(|| {
        let sys_info = system::server_info::get_oper_sys_info();
        Event::default().data(serde_json::to_string(&sys_info).unwrap_or_else(|_| "0".to_string()))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    Sse::new(stream).keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(1)).text("keep-alive-text"))
}
