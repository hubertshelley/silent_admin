use silent::{Request, Result};

use dto::common::captcha::CaptchaImage;
use services::system;

pub async fn get_captcha(_req: Request) -> Result<CaptchaImage> {
    let res = system::common::get_captcha();
    Ok(res)
}
