use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use crate::rest_api::base::{BaseInfo, ops::BaseOps, NameType, Ideantifier};
use crate::rest_api::base::wrap::ActiveRecordWrap;
use crate::rest_api::json_models::issue::IssueDto;
use crate::rest_api::service::issues::{fetch_issue_by_id, persist_changes};
use std::ops::{Deref, DerefMut};
use std::sync::Arc;
use crate::rest_api::json_models::issue::field::custom_field::{IssueCustomField, IssueStatus, StateIssueCustomField};
use crate::rest_api::json_models::issue::field::value::{FieldValue, StateBundleElement};
use crate::rest_api::json_models::issue::field::IssueStateType;

pub type Issue = ActiveRecordWrap<IssueDto>;

#[async_trait]
pub trait IssueContract: BaseInfo + BaseOps + Sync {
    // async fn fields(&self) -> Vec<Box<IssueCustomField>> where Self: Sized;
    // async fn status(&self) -> Box<IssueCustomField> where Self: Sized;
    // async fn set_state_name(&mut self, status_name:String) -> () {
    //     self.
    //     match self.value {
    //         FieldValue::StateBundleElement {
    //             ref mut name,
    //             ..
    //         } => {
    //             *name = Some(status_name)
    //         },
    //         _ => ()
    //     };
    // }
    // async fn owner(&self) -> Vec<Box<dyn User>>;
}

// #[async_trait]
// impl IssueContract for Issue {
//     async fn fields(&self) -> Vec<Box<IssueCustomField>> {
//         unimplemented!()
//     }
//
//     async fn status(&self) -> Box<IssueStatus> {
//
//     }
// }

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
        let new_origin = persist_changes(&self.http_client, self.origin.clone(), self.inner.clone()).await;
        self.refresh(new_origin);
        self
    }
}

impl Issue {
    pub fn set_state_name(&mut self, status_name: String) {
        let dto = &*self.inner;
        let new_fields = {
            let mut cloned_fields = dto.fields.clone();
            let (index, new_field) = cloned_fields.iter().enumerate()
                .filter_map(|(index, custom_field)| {
                    match custom_field.clone() {
                        IssueCustomField::StateIssueCustomField(state_custom_field) =>
                            match state_custom_field.value.clone() {
                                FieldValue::StateBundleElement(mut state_bundle_element) => {
                                    state_bundle_element.name = Some(status_name.clone());
                                    let new_field = IssueCustomField::StateIssueCustomField(
                                        StateIssueCustomField {
                                            value: FieldValue::StateBundleElement(state_bundle_element),
                                            ..state_custom_field
                                        }
                                    );
                                    Some((index, new_field))
                                }
                                _ => None
                            }
                        _ => None
                    }
                }).next()
                .unwrap();
            cloned_fields.remove(index);
            cloned_fields.push(new_field);
            cloned_fields
        };

        let new_mutable_state = IssueDto { fields: new_fields, ..dto.clone() };
        *Arc::make_mut(&mut self.inner) = new_mutable_state;
    }
    pub fn set_state(&mut self, state_type: IssueStateType) {
        self.set_state_name(state_type.into())
    }
}
