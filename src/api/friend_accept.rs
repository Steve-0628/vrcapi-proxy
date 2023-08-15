use super::utils::{request, StrExt as _};
use crate::general::find_matched_data;
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum Response {
    Success,
    Error(String),
}

#[post("/friend_accept", data = "<req>")]
pub(crate) async fn api_friend_accept(req: &str) -> (Status, Json<Response>) {
    match fetch(req).await {
        Ok(_) => (Status::Ok, Json(Response::Success)),

        Err(error) => (
            Status::InternalServerError,
            Json(Response::Error(error.to_string())),
        ),
    }
}

async fn fetch(req: &str) -> Result<()> {
    let (auth, id) = req.split_colon()?;

    let matched = find_matched_data(auth)?;

    let res = request(
        reqwest::Method::PUT,
        &format!("https://api.vrchat.cloud/api/1/auth/user/notifications/{id}/accept"),
        &matched.token,
    )
    .await?;

    if res.status().is_success() {
        Ok(())
    } else {
        bail!("{}", res.status())
    }
}
