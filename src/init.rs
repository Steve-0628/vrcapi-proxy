use crate::{
    global::DATA_PATH,
    json::{read_json, write_json},
};
use anyhow::{anyhow, ensure, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub(super) struct Data {
    pub(super) listen: String,
    pub(super) auth: String,
    pub(super) token: String,
}

pub(super) fn init() -> Result<()> {
    if let Ok(data) = read_json::<Data>("data.json") {
        ensure!(
            !data.auth.is_empty(),
            "認証IDが空です。入力して再度起動してください。"
        );
        return Ok(());
    }

    let default = Data {
        listen: "0.0.0.0:8000".into(),
        auth: String::new(),
        token: String::new(),
    };

    write_json(&default, "data.json")?;

    Err(anyhow!(
        "{} を生成しました。設定後再度起動してください。",
        DATA_PATH.display()
    ))
}
