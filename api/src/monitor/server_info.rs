use dto::system::server_info::SysInfo;
use futures::stream;
use services::system;
use silent::prelude::{sse_reply, SSEEvent};
use silent::{Request, Response};
use std::time::Duration;
use tokio_stream::StreamExt;

pub async fn get_server_info(_req: Request) -> silent::Result<SysInfo> {
    let res = system::server_info::get_oper_sys_info();

    Ok(res)
}

pub async fn get_server_info_sse(_req: Request) -> silent::Result<Response> {
    let stream = stream::repeat_with(|| {
        let sys_info = system::server_info::get_oper_sys_info();
        SSEEvent::default()
            .data(serde_json::to_string(&sys_info).unwrap_or_else(|_| "0".to_string()))
    })
    .map(Ok)
    .throttle(Duration::from_secs(1));

    sse_reply(stream)
}
