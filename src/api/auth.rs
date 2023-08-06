use anyhow::{bail, Context as _, Result};
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

const URL: &str = "https://api.vrchat.cloud/api/1/auth/user";

const ON_ERROR: &str = "An error occurred while parsing the cookie.";

#[derive(Serialize)]
enum Token {
    Success { token: String },
    Error { error: String },
}

#[post("/auth", data = "<req>")]
pub(crate) async fn api_auth(req: &str) -> String {
    let result = match auth(req).await {
        Ok(t) => Token::Success { token: t },
        Err(error) => Token::Error {
            error: error.to_string(),
        },
    };

    serde_json::to_string(&result).unwrap()
}

async fn auth(req: &str) -> Result<String> {
    let client = reqwest::Client::new();

    let auth_header = format!("Basic {}", general_purpose::STANDARD_NO_PAD.encode(req));

    let response = client
        .get(URL)
        .header("Authorization", auth_header)
        .header("User-Agent", "vrc-rs")
        .send()
        .await?;

    if response.status().is_success() {
        Ok(response
            .headers()
            .get("set-cookie")
            .context(ON_ERROR)?
            .to_str()?
            .split(';')
            .next()
            .context(ON_ERROR)?
            .split('=')
            .nth(1)
            .context(ON_ERROR)?
            .to_owned())
    } else {
        bail!("Error: status code: {}", response.status())
    }
}
