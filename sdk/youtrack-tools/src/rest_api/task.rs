use async_trait::async_trait;
use crate::rest_api::base::BaseInfo;

#[async_trait]
pub trait Task: BaseInfo {
    // async fn users(&self) -> Vec<Box<dyn User>>;
}