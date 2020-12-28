use crate::rest_api::base::BaseInfo;
use crate::rest_api::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait Project: BaseInfo {
    async fn users(&self) -> Vec<Box<dyn User>>;
}