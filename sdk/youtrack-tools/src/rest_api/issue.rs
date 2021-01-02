use async_trait::async_trait;
use crate::rest_api::base::{BaseInfo, BaseOps, NameType, Ideantifier};
use crate::rest_api::issue::field::{IssueDynamicField, IssueStatus};

#[async_trait]
pub trait Issue: BaseInfo + BaseOps + Sync {
    async fn fields(&self) -> Vec<Box<IssueDynamicField>> where Self: Sized;
    async fn status(&self) -> Box<IssueStatus> where Self: Sized;

    // async fn owner(&self) -> Vec<Box<dyn User>>;
}

mod field {
    pub enum IssueDynamicField {
        SimpleString,
        SimpleText,
        SimpleDate,
        SimpleDateTime,
        SimplePeriod,
        SimpleInteger,
        SimpleFloat,
        EnumeratedEnum,
        EnumeratedGroup,
        EnumeratedUser,
        EnumeratedOwnerField,
        EnumeratedState,
        EnumeratedVersion,
        EnumeratedBuild,
    }

    // pub trait TaskDynamicField {
    //     fn name(&self)->String;
    // }
    //
    // pub struct TaskStatus {}
    //
    // impl TaskDynamicField for TaskStatus {
    //     fn name(&self) -> String {
    //         unimplemented!()
    //     }
    // }
    pub type IssueStatus = IssueDynamicField;
}

pub struct IssueImpl {

}

#[async_trait]
impl Issue for IssueImpl {
    async fn fields(&self) -> Vec<Box<IssueDynamicField>> {
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