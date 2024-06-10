use anyhow::Result;
use captcha_rust::Captcha;
use configs::CFG;
use db::common::captcha::CaptchaImage;
use http::StatusCode;
use silent::prelude::FilePart;
use silent::SilentError;
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

/// 获取验证码
pub fn get_captcha() -> CaptchaImage {
    let captcha = Captcha::new(4, 130, 40);
    let uuid = utils::encrypt_password(&captcha.text, "");
    CaptchaImage {
        captcha_on_off: true,
        uuid,
        img: captcha.base_img,
    }
}

fn get_file_type(content_type: &str) -> String {
    match content_type {
        "image/jpeg" => ".jpg".to_string(),
        "image/png" => ".png".to_string(),
        "image/gif" => ".gif".to_string(),
        _ => "".to_string(),
    }
}

/// 上传相关
pub async fn upload_file(file_part: &FilePart) -> Result<String> {
    let res: Result<String> = {
        let file_type = file_part.name().unwrap_or_else(|| "").split(".").last().unwrap_or_else(|| "");
        let now = chrono::Local::now();
        let file_path_t = CFG.web.upload_dir.clone() + "/" + &now.format("%Y-%m").to_string();
        let url_path_t = CFG.web.upload_url.clone() + "/" + &now.format("%Y-%m").to_string();
        fs::create_dir_all(&file_path_t).await?;
        let file_name = now.format("%d").to_string() + "-" + &scru128::new_string() + "." + &file_type;
        let file_path = file_path_t + "/" + &file_name;
        let url_path = url_path_t + "/" + &file_name;
        // todo: 优化异步上传
        // file_part.save(file_path)?;
        std::fs::copy(file_part.path(), Path::new(file_path.as_str())).map_err(|e| SilentError::BusinessError {
            code: StatusCode::INTERNAL_SERVER_ERROR,
            msg: format!("Failed to save file: {}", e),
        })?;
        Ok(url_path)
    };
    if let Ok(url_path) = res {
        Ok(url_path)
    } else {
        Err(anyhow::anyhow!("上传文件失败"))
    }
}

/// 删除文件
pub async fn delete_file(file_path: &str) {
    let path = file_path.replace(&CFG.web.upload_url, &CFG.web.upload_dir);
    match fs::remove_file(&path).await {
        Ok(_) => {}
        Err(_) => {
            tracing::error!("删除文件失败:{}", path);
        }
    }
}
