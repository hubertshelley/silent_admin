use crate::jwt::{Claims, JWT_KEY};
use crate::SilentAdminError;
use configs::CFG;
use jsonwebtoken::{decode, DecodingKey, Validation};
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
            id: value.id.clone(),
            username: value.name.clone(),
            claims: value,
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
    pub fn id(&self) -> Result<String, SilentAdminError> {
        match self {
            User::Authenticated(auth_user) => Ok(auth_user.id.clone()),
            User::Anonymous => Err(SilentAdminError::Unauthorized),
        }
    }
    pub fn username(&self) -> Result<String, SilentAdminError> {
        match self {
            User::Authenticated(auth_user) => Ok(auth_user.username.clone()),
            User::Anonymous => Err(SilentAdminError::Unauthorized),
        }
    }
    pub fn claims(&self) -> Result<Claims, SilentAdminError> {
        match self {
            User::Authenticated(auth_user) => Ok(auth_user.claims.clone()),
            User::Anonymous => Err(SilentAdminError::Unauthorized),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: String,
    pub username: String,
    pub claims: Claims,
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
        let algorithm: Option<String> = CFG.jwt.jwt_algorithm.clone();
        let issuer: Option<String> = CFG.jwt.jwt_issuer.clone();
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
