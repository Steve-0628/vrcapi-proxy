mod auth;
mod friend_request;
mod friend_status;
mod friends;
mod instance;
mod search_user;
mod toggle_askme;
mod two_factor_email;
mod user;

pub(crate) use auth::api_auth;
pub(crate) use friend_request::{api_del_friend_request, api_friend_request};
pub(crate) use friend_status::api_friend_status;
pub(crate) use friends::api_friends;
pub(crate) use instance::api_instance;
pub(crate) use search_user::api_search_user;
pub(crate) use toggle_askme::api_toggle;
pub(crate) use two_factor_email::api_twofactor_email;
pub(crate) use user::api_user;
