use crate::rest_api::base::BaseInfo;
use async_trait::async_trait;

#[async_trait]
pub trait User: BaseInfo {}