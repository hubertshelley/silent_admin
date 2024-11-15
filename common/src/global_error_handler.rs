use crate::response::Res;
use silent::{Configs, Response, Result};

pub async fn exception_handler(res: Result<Response>, _configs: Configs) -> Result<Response> {
    Ok(res.unwrap_or_else(|err| {
        Res::<()>::with_err(err.status().as_u16(), err.message().as_str()).into()
    }))
}
