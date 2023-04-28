use crate::auth::Auth;
use chrono::{Duration, Utc};
use serde::Serialize;

type Url = String;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    #[serde(skip_serializing)]
    pub hash: String,
}

#[derive(Serialize)]
pub struct UserAuth<'a> {
    email: &'a str,
    token: String,
}

impl User {
    pub fn to_user_auth(&self, secret: &[u8]) -> UserAuth {
        let exp = Utc::now() + Duration::days(10); 
        let token = Auth {
            id: self.id,
            exp: exp.timestamp(),
        }
        .token(secret);

        UserAuth {
            email: &self.email,
            token,
        }
    }
}
