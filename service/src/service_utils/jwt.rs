use chrono::{Duration, Local};
use db::common::ctx::UserInfoCtx;
use headers::{authorization::Bearer, Authorization, HeaderMapExt};
use http::request::Parts;
use http::{HeaderMap, HeaderValue, StatusCode};
use jsonwebtoken::{decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use silent::{Request, Response, Result as SilentResult, SilentError};

use super::super::system::check_user_online;

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &CFG.jwt.jwt_secret;
    Keys::new(secret.as_bytes())
});
use configs::CFG;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthPayload {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: String,
    pub token_id: String,
    pub name: String,
    pub exp: i64,
}

impl Claims {
    /// 将用户信息注入request
    pub async fn from_request_parts(req: &mut Request) -> SilentResult<Self> {
        let (_, token_v) = get_bear_token(req.headers_mut()).await.map_err(|e| <AuthError as Into<SilentError>>::into(e))?;
        // Decode the user data

        let token_data = match decode::<Claims>(&token_v, &KEYS.decoding, &Validation::default()) {
            Ok(token) => {
                let token_id = token.claims.token_id.clone();
                let (x, _) = check_user_online(None, token_id.clone()).await;
                if x {
                    token
                } else {
                    return Err(AuthError::CheckOutToken.into());
                }
            }
            Err(err) => {
                return match *err.kind() {
                    ErrorKind::InvalidToken => Err(AuthError::InvalidToken.into()),
                    ErrorKind::ExpiredSignature => Err(AuthError::MissingCredentials.into()),
                    _ => Err(AuthError::WrongCredentials.into()),
                }
            }
        };
        let user = token_data.claims;
        req.extensions_mut().insert(UserInfoCtx {
            id: user.id.clone(),
            token_id: user.token_id.clone(),
            name: user.name.clone(),
        });
        Ok(user)
    }
}

pub async fn get_bear_token(headers: &mut HeaderMap<HeaderValue>) -> Result<(String, String), AuthError> {
    // Extract the token from the authorization header
    let Authorization(bearer) = headers
        .typed_try_get::<Authorization<Bearer>>()
        .map_err(|_| AuthError::InvalidToken)?
        .ok_or(AuthError::InvalidToken)?;
    // Decode the user data
    let bearer_data = bearer.token();
    let cut = bearer_data.len() - scru128::new_string().len();
    Ok((bearer_data[cut..].to_string(), bearer_data[0..cut].to_string()))
}

pub async fn authorize(payload: AuthPayload, token_id: String) -> Result<AuthBody, AuthError> {
    if payload.id.is_empty() || payload.name.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let iat = Local::now();
    let exp = iat + Duration::minutes(CFG.jwt.jwt_exp);
    let claims = Claims {
        id: payload.id.to_owned(),
        token_id: token_id.clone(),
        name: payload.name,
        exp: exp.timestamp(),
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| AuthError::WrongCredentials)?;
    // Send the authorized token
    Ok(AuthBody::new(token, claims.exp, CFG.jwt.jwt_exp, token_id))
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    CheckOutToken,
}
impl Into<SilentError> for AuthError {
    fn into(self) -> SilentError {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
            AuthError::CheckOutToken => (StatusCode::UNAUTHORIZED, "该账户已经退出"),
        };
        SilentError::business_error(status, json!({ "error": error_message }).to_string())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    token: String,
    token_type: String,
    pub exp: i64,
    exp_in: i64,
}
impl AuthBody {
    fn new(access_token: String, exp: i64, exp_in: i64, token_id: String) -> Self {
        Self {
            token: access_token + &token_id,
            token_type: "Bearer".to_string(),
            exp,
            exp_in,
        }
    }
}
