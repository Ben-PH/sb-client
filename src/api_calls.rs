use crate::User;
use seed::{log, prelude::*};

pub async fn check_login() -> fetch::Result<User> {
    log!("");
    Request::new("api/auth")
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn login_resp(resp: fetch::Result<Response>) -> fetch::Result<User> {
    resp?.json::<User>().await
}

pub async fn post_logout(user: &User) -> fetch::Result<Response> {
    Request::new("api/auth")
        .method(Method::Delete)
        .text(&user.email)
        .fetch()
        .await?
        .check_status()
}
