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

