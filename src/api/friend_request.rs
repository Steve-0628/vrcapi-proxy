use super::utils::{find_matched_data, request};
use crate::{api::response::ApiResponse, split_colon};

#[post("/friend_request", data = "<req>")]
pub(crate) fn api_friend_request(req: &str) -> ApiResponse<bool> {
    (|| {
        split_colon!(req, [auth, user, method]);

        let token = find_matched_data(auth)?.1;

        request(
            method,
            &format!("https://api.vrchat.cloud/api/1/user/{}/friendRequest", user),
            &token,
        )?;

        Ok(true)
    })()
    .into()
}
