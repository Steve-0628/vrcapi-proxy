use super::utils::{find_matched_data, request};
use crate::split_colon;
use anyhow::Result;

#[post("/friend_request", data = "<req>")]
pub(crate) fn api_friend_request(req: &str) -> Result<bool> {
    split_colon!(req, [auth, user, method]);

    let token = find_matched_data(auth)?.1;

    request(
        method,
        &format!("https://api.vrchat.cloud/api/1/user/{}/friendRequest", user),
        &token,
    )
    .map(|_| true)
}
