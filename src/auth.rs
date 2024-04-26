use axum::{
    extract::FromRequestParts,
    http::{header, StatusCode, request::Parts},
    async_trait,
};
use log::debug;
use data_encoding::BASE64;

#[derive(PartialEq, Debug)]
pub enum RequireAuth {
    Basic {
        name: String,
        password: String,
    },
    Token {
        name: String,
        token: String,
    },
    None,
}

impl Default for RequireAuth {
    fn default() -> Self {
        Self::None
    }
}

impl RequireAuth {
    pub fn is_some(&self) -> bool {
        !matches!(*self, RequireAuth::None)
    }
    pub fn is_none(&self) -> bool {
        matches!(*self, RequireAuth::None)
    }
}

// pub struct User {
//     pub id: i32,
//     pub name: String,
//     pub rank: UserRank,
// }

#[async_trait]
impl<S> FromRequestParts<S> for RequireAuth
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|value| value.to_str().ok());
        // let auth_cookie; TODO: Узнать на счёт использования кукесов для авторизации
        debug!("!AUTH {auth_header:?}");

        if auth_header.is_none()  {
            debug!("Don't have Auth: {auth_header:?}");
            return Ok(Self::default())
        }
        let (metod, auth) = match auth_header.unwrap().split_once(' ') {
            Some((metod, auth)) => (metod, auth),
            None => ("", auth_header.unwrap()), // В метод ничего, а в ауз то что было в голове.
        };

        match metod {
            "Basic" => {
                let (name, password) = auth.split_once(':').unwrap();
                debug!("!BASIC {name} {password}");
                Ok(Self::Basic {name: name.to_string(), password: password.to_string()})
            }
            "Token" => {
                let auth = String::from_utf8(BASE64.decode(auth.as_bytes()).unwrap()).unwrap();
                let (name, token) = auth.split_once(':').unwrap();
                debug!("!TOKEN {name} {token}");
                Ok(Self::Token { name: name.to_string(), token: token.to_string() })
            }
            _ => { // Попадём сюда в случае если метод пустой, когда там неверные данные например, или если что-то другое.
                // Boom!
                debug!("Something other in auth: {auth}");
                Ok(Self::default())
            }
        }

        // debug!("Auth: {auth_header:?}");
        // Ok(Self::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::RequireAuth;

    #[test]
    fn is_some() {
        let x = RequireAuth::Basic { name: "".to_string(), password: "".to_string() };
        assert_eq!(x.is_some(), true)
    }
    #[test]
    fn is_none() {
        let x = RequireAuth::None;
        assert_eq!(x.is_none(), true)
    }
}