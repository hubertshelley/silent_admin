use crate::jwt::{Claims, JWT_KEY};
use crate::BpmError;
use jsonwebtoken::{decode, DecodingKey, Validation};
use sea_orm::prelude::async_trait;
use silent::headers::authorization::Bearer;
use silent::headers::{Authorization, HeaderMapExt};
use silent::prelude::Next;
use silent::{Handler, MiddleWareHandler, Request, Response, SilentError};

#[derive(Clone, Debug)]
pub enum User {
    Anonymous,
    Authenticated(AuthUser),
}

impl From<Claims> for User {
    fn from(value: Claims) -> Self {
        Self::Authenticated(AuthUser {
            id: value.id,
            username: value.name,
        })
    }
}

impl User {
    pub fn is_authenticated(&self) -> bool {
        match self {
            User::Authenticated(_) => true,
            User::Anonymous => false,
        }
    }
    pub fn id(&self) -> Result<String, BpmError> {
        match self {
            User::Authenticated(auth_user) => Ok(auth_user.id.clone()),
            User::Anonymous => Err(BpmError::Unauthorized),
        }
    }
    pub fn username(&self) -> Result<String, BpmError> {
        match self {
            User::Authenticated(auth_user) => Ok(auth_user.username.clone()),
            User::Anonymous => Err(BpmError::Unauthorized),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
}

#[derive(Clone)]
pub struct JWTAuthorizationMiddleware {
    secret_key: DecodingKey,
    validation: Validation,
}

impl Default for JWTAuthorizationMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl JWTAuthorizationMiddleware {
    pub fn new() -> Self {
        let algorithm = std::env::var("JWT_ALGORITHM").ok();
        let issuer = std::env::var("JWT_ISSUER").ok();
        let secret_key = JWT_KEY.decoding.clone();
        let mut validation = if let Some(algorithm) = algorithm {
            Validation::new(algorithm.parse().expect("Invalid algorithm"))
        } else {
            Validation::default()
        };
        if let Some(issuer) = issuer {
            validation.set_issuer(vec![issuer].as_slice())
        }
        Self {
            secret_key,
            validation,
        }
    }
}

#[async_trait::async_trait]
impl MiddleWareHandler for JWTAuthorizationMiddleware {
    async fn handle(&self, mut req: Request, next: &Next) -> Result<Response, SilentError> {
        let user = if let Ok(Some(Authorization(bearer))) =
            req.headers_mut().typed_try_get::<Authorization<Bearer>>()
        {
            match decode::<Claims>(bearer.token(), &self.secret_key, &self.validation) {
                Ok(token) => token.claims.into(),
                Err(_) => User::Anonymous,
            }
        } else {
            User::Anonymous
        };
        req.extensions_mut().insert(user.clone());
        let mut res = next.call(req).await?;
        res.extensions_mut().insert(user);
        Ok(res)
    }
}
