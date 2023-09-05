use super::utils::{find_matched_data, request};
use crate::{api::response::ApiResponse, consts::VRC_P, split_colon};
use anyhow::Result;
use serde::{Deserialize, Serialize};

const URL: &str = "https://api.vrchat.cloud/api/1/users?search=";

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct User {
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    id: String,
    isFriend: bool,
    statusDescription: String,
    tags: Vec<String>,
    userIcon: String,
    profilePicOverride: String,
}

#[allow(non_snake_case)]
#[derive(Serialize)]
pub(crate) struct ResUser {
    currentAvatarThumbnailImageUrl: String,
    displayName: String,
    id: String,
    isFriend: bool,
    statusDescription: String,
}

impl From<User> for ResUser {
    fn from(user: User) -> Self {
        let img = match user.tags.iter().any(|tag| tag == VRC_P) {
            true if !user.userIcon.is_empty() => user.userIcon,
            true if !user.profilePicOverride.is_empty() => user.profilePicOverride,
            _ => user.currentAvatarThumbnailImageUrl,
        };

        ResUser {
            currentAvatarThumbnailImageUrl: img,
            displayName: user.displayName,
            id: user.id,
            isFriend: user.isFriend,
            statusDescription: user.statusDescription,
        }
    }
}

#[post("/search_user", data = "<req>")]
pub(crate) fn api_search_user(req: &str) -> ApiResponse<Vec<ResUser>> {
    fetch(req).into()
}

fn fetch(req: &str) -> Result<Vec<ResUser>> {
    split_colon!(req, [auth, user]);

    let (_, token) = find_matched_data(auth)?;

    request("GET", &format!("{}{}", URL, user), &token).map(|res| {
        Ok(res
            .into_json::<Vec<User>>()?
            .into_iter()
            .map(ResUser::from)
            .collect::<Vec<_>>())
    })?
}
