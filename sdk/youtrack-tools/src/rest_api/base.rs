use async_trait::async_trait;

pub type Ideantifier = String;
pub type NameType = String;

#[async_trait]
pub trait BaseInfo {
    async fn name(&self) -> NameType;
    async fn id(&self) -> Ideantifier;
}