use youtrack_tools::rest_api::client::{YoutrackClientImpl, YoutrackClient};
use std::sync::Arc;
use gitlab_tools::models::hooks::GitlabHookRequest;
use youtrack_tools::rest_api::json_models::issue::field::IssueStateType;

pub struct YoutrackService {
    client: Arc<YoutrackClientImpl>
}

impl YoutrackService {
    pub fn new(client: YoutrackClientImpl) -> YoutrackService {
        let client = Arc::new(client);
        YoutrackService { client }
    }

    pub fn process_web_hook(&mut self, hook: GitlabHookRequest) {

    }

    pub async fn update_status(&mut self, task_id:String) {
        let mut issue = self.client.issue(task_id).await;
        if let Some(state)=issue.get_state(){
            // if state.value. == Sta {
            //     // IssueStateType::Fixed
            //     // parse from config
            //     issue.set_state(IssueStateType::Fixed)
            // }
        }

    }
}
