use crate::{
    data::{Data, DataVecExt as _},
    general::update_data_property,
};
use anyhow::{bail, Context as _, Error, Result};
use rocket::serde::json::Json;
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/twofactorauth/emailotp/verify";

#[derive(Serialize)]
pub(crate) enum Res {
    Success(String),
    Error(String),
}

impl From<Error> for Res {
    fn from(error: Error) -> Self {
        Res::Error(error.to_string())
    }
}

#[post("/twofactor_email", data = "<req>")]
pub(crate) async fn api_twofactor_email(req: &str) -> Json<Res> {
    let result = match req.split_once(';') {
        Some((req, auth)) => match fetch(req).await {
            Ok(token) => {
                if let Err(err) = update(token, auth) {
                    return Json(Res::from(err));
                }

                Res::Success(auth.to_string())
            }
            Err(err) => Res::from(err),
        },
        None => match fetch(req).await {
            Ok(token) => {
                let auth = Uuid::new_v4().to_string();

                if let Err(err) = add(token, &auth) {
                    return Json(Res::from(err));
                }

                Res::Success(auth)
            }
            Err(err) => Res::from(err),
        },
    };

    Json(result)
}

async fn fetch(req: &str) -> Result<&str> {
    let (token, f) = req.split_once(':').context("Unexpected input.")?;
    let res = reqwest::Client::new()
        .post(URL)
        .header("User-Agent", "vrc-rs")
        .header("Cookie", token)
        .json(&json!({ "code": f }))
        .send()
        .await?;

    if res.status().is_success() {
        Ok(token)
    } else {
        bail!("Error: {}", res.status())
    }
}

fn update(token: &str, auth: &str) -> Result<()> {
    update_data_property(auth, |data| {
        data.token = token.to_string();
    })?;

    Ok(())
}

fn add(token: &str, auth: &str) -> Result<()> {
    let new_data = Data {
        auth: auth.to_string(),
        token: token.to_string(),
        askme: false,
    };

    let mut data: Vec<Data> = Data::get()?;

    data.push(new_data);

    data.write()?;

    Ok(())
}
