use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use crate::rest_api::base::{BaseInfo, ops::BaseOps, NameType, Ideantifier};
use crate::rest_api::base::wrap::ActiveRecordWrap;
use crate::rest_api::json_models::issue::IssueDto;
use crate::rest_api::service::issues::{fetch_issue_by_id, persist_changes};
use std::ops::Deref;
use std::sync::Arc;
use crate::rest_api::json_models::issue::field::custom_field::{IssueCustomField, IssueStatus};

pub type Issue = ActiveRecordWrap<IssueDto>;

#[async_trait]
pub trait IssueContract: BaseInfo + BaseOps + Sync {
    async fn fields(&self) -> Vec<Box<IssueCustomField>> where Self: Sized;
    async fn status(&self) -> Box<IssueCustomField> where Self: Sized;

    // async fn owner(&self) -> Vec<Box<dyn User>>;
}

#[async_trait]
impl IssueContract for Issue {
    async fn fields(&self) -> Vec<Box<IssueCustomField>> {
        unimplemented!()
    }

    async fn status(&self) -> Box<IssueStatus> {
        unimplemented!()
    }
}

#[async_trait]
impl BaseInfo for Issue {
    async fn name(&self) -> NameType {
        unimplemented!()
    }

    async fn id(&self) -> Ideantifier {
        unimplemented!()
    }
}

#[async_trait]
impl BaseOps for Issue {
    async fn update(&mut self) -> &mut Self {
        let new_origin = fetch_issue_by_id(&self.http_client, self.origin.id.clone()).await;
        self.refresh(new_origin);
        self
    }

    async fn save(&mut self) -> &mut Self {
        let new_origin = persist_changes(&self.http_client,  self.origin.clone(),self.inner.clone()).await;
        self.refresh(new_origin);
        self
    }
}
