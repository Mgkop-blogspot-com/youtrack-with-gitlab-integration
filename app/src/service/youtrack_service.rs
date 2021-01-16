use youtrack_tools::rest_api::client::{YoutrackClientImpl, YoutrackClient};
use std::sync::Arc;
use gitlab_tools::models::hooks::GitlabHookRequest;
use youtrack_tools::rest_api::json_models::issue::field::IssueStateType;
use youtrack_tools::rest_api::json_models::issue::field::value::{FieldValue, StateBundleElement};
use youtrack_tools::rest_api::base::ops::BaseOps;
use youtrack_tools::rest_api::json_models::issue::IssueDto;
use youtrack_tools::rest_api::base::wrap::ActiveRecordWrap;
use youtrack_tools::rest_api::json_models::issue::field::custom_field::StateIssueCustomField;
use youtrack_tools::rest_api::error::Result as Res;
use youtrack_tools::rest_api::error::Error as YoutrackError;

pub struct YoutrackService {
    client: Arc<YoutrackClientImpl>
}

impl YoutrackService {
    pub fn new(client: YoutrackClientImpl) -> YoutrackService {
        let client = Arc::new(client);
        YoutrackService { client }
    }

    pub async fn update_status(&mut self, task_id: String) {
        match self.client.issue(task_id).await {
            Ok(mut issue)=>{
                if let Some(state) = issue.get_state() {
                    if let FieldValue::StateBundleElement(StateBundleElement { name: Some(IssueStateType::WaitForMerge), .. }) = state.value {
                        issue.set_state(IssueStateType::Fixed);
                    }
                    // if state.value. == Sta {
                    //     // IssueStateType::Fixed
                    //     // parse from config
                    //     issue.set_state(IssueStateType::Fixed)
                    // }
                }
                issue.save().await;
            },
            Err(e) => {
                log::error!("{}", e)
            }
        }
    }
}
