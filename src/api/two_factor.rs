use crate::{
    fetcher::request_json, global::AUTHORIZATION, init::Data, validate::validate,
    websocket::spawn_client::spawn_ws_client,
};
use anyhow::Result;
use axum::Json;
use hyper::Method;
use serde_json::json;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub(crate) struct Query {
    auth: String,
    token: String,
    two_factor_type: String, // emailotp | totp | otp
    two_factor_code: String,
}

pub(crate) async fn api_twofactor(
    Json(Query {
        auth,
        token,
        two_factor_code,
        two_factor_type,
    }): Json<Query>,
) -> Result<bool> {
    drop(validate(auth)?);

    request_json(
        Method::POST,
        &format!("https://api.vrchat.cloud/api/1/auth/twofactorauth/{two_factor_type}/verify"),
        &token,
        json!({ "code": two_factor_code }),
    )
    .await?;

    let id = Uuid::new_v4().to_string();
    let data = {
        let Data { listen, auth, .. } = crate::json::read_json("data.json")?;
        Data {
            listen,
            auth,
            token,
        }
    };

    crate::json::write_json::<Data>(&data, "data.json")?;

    *AUTHORIZATION.1.write().await = data.token;

    spawn_ws_client().await;

    Ok(true)
}
