use super::utils::CLIENT;
use crate::{
    api::response::ApiResponse,
    consts::{COOKIE, UA, UA_VALUE},
    general::{read_json, HashMapExt as _},
    into_err, spawn, split_colon,
};
use anyhow::{bail, Result};
use rocket::{http::Status, serde::json::Json};
use serde_json::json;
use std::collections::HashMap;
use uuid::Uuid;

#[post("/twofactor", data = "<req>")]
pub(crate) async fn api_twofactor(req: &str) -> (Status, Json<ApiResponse<String>>) {
    match req.split_once(';') {
        Some((req, auth)) => match fetch(req).await {
            Ok(token) => {
                if let Err(err) = update(auth, token) {
                    return (Status::InternalServerError, Json(into_err!(err)));
                }

                (Status::Ok, Json(auth.into()))
            }

            Err(err) => (Status::InternalServerError, Json(into_err!(err))),
        },

        None => match fetch(req).await {
            Ok(token) => {
                let auth = Uuid::new_v4().to_string();

                if let Err(err) = update(&auth, token) {
                    return (Status::InternalServerError, Json(into_err!(err)));
                }

                (Status::Ok, Json(auth.into()))
            }

            Err(err) => (Status::InternalServerError, Json(into_err!(err))),
        },
    }
}

async fn fetch(req: &str) -> Result<&str> {
    split_colon!(req, [token, r#type, f]);

    let res = CLIENT
        .post(format!(
            "https://api.vrchat.cloud/api/1/auth/twofactorauth/{type}/verify"
        ))
        .header(UA, UA_VALUE)
        .header(COOKIE, token)
        .json(&json!({ "code": f }))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(token)
    } else {
        bail!("{}", res.text().await?)
    }
}

fn update(auth: &str, token: &str) -> Result<()> {
    let mut data: HashMap<String, String> = read_json("data.json")?;

    data.add(auth, token)?;

    spawn(unsafe { data.remove_entry(auth).unwrap_unchecked() });

    Ok(())
}
