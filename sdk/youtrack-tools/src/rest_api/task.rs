use async_trait::async_trait;
use crate::rest_api::base::{BaseInfo, BaseOps, NameType, Ideantifier};
use crate::rest_api::task::field::{TaskDynamicField, TaskStatus};

#[async_trait]
pub trait Task: BaseInfo + BaseOps + Sync {
    async fn fields(&self) -> Vec<Box<dyn TaskDynamicField>> where Self: Sized;
    async fn status(&self) -> Box<TaskStatus> where Self: Sized;

    // async fn owner(&self) -> Vec<Box<dyn User>>;
}
pub struct TaskImpl {

}

#[async_trait]
impl Task for TaskImpl {
    async fn fields(&self) -> Vec<Box<dyn TaskDynamicField>> {
        unimplemented!()
    }

    async fn status(&self) -> Box<TaskStatus> {
        unimplemented!()
    }
}

#[async_trait]
impl BaseInfo for TaskImpl {
    async fn name(&self) -> NameType {
        unimplemented!()
    }

    async fn id(&self) -> Ideantifier {
        unimplemented!()
    }
}

#[async_trait]
impl BaseOps for TaskImpl {
    async fn update(&self) -> Self {
        unimplemented!()
    }
}