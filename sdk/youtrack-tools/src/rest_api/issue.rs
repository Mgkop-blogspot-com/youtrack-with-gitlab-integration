use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use crate::rest_api::base::{BaseInfo, BaseOps, NameType, Ideantifier};

pub type IssueStatus = String;
pub type IssueCustomField = String;

#[async_trait]
pub trait Issue: BaseInfo + BaseOps + Sync {
    async fn fields(&self) -> Vec<Box<IssueCustomField>> where Self: Sized;
    async fn status(&self) -> Box<IssueCustomField> where Self: Sized;

    // async fn owner(&self) -> Vec<Box<dyn User>>;
}

pub struct IssueImpl {}

#[async_trait]
impl Issue for IssueImpl {
    async fn fields(&self) -> Vec<Box<IssueCustomField>> {
        unimplemented!()
    }

    async fn status(&self) -> Box<IssueStatus> {
        unimplemented!()
    }
}

#[async_trait]
impl BaseInfo for IssueImpl {
    async fn name(&self) -> NameType {
        unimplemented!()
    }

    async fn id(&self) -> Ideantifier {
        unimplemented!()
    }
}

#[async_trait]
impl BaseOps for IssueImpl {
    async fn update(&self) -> Self {
        unimplemented!()
    }
}
