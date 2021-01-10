use youtrack_tools::rest_api::client::YoutrackClientImpl;
use std::sync::Arc;
use gitlab_tools::models::hooks::GitlabHookRequest;
use crate::service::youtrack_service::YoutrackService;
use std::error::Error;
use gitlab_tools::models::hooks::merge_request::MergeRequestHook;
use gitlab_tools::models::hooks::merge_request::custom::{MergeRequestState, MergeRequestAction};
use std::borrow::BorrowMut;
use tokio::sync::RwLock;
use snafu::{ensure, Backtrace, ErrorCompat, ResultExt, Snafu};

/// This implementation doesn't support grok patterns
pub struct SimpleWebhookService {
    youtrack_service: Arc<RwLock<YoutrackService>>
}

// #[derive(Debug, Snafu)]
#[derive(Debug)]
pub enum SimpleWebhookServiceError {
    // #[snafu(display("Note hook not yet implemented"))]
    NoteHookNotImplemented,
    // #[snafu(display("Pipeline hook not yet implemented"))]
    PipelineHookNotImplemented,
}

lazy_static! {
	 static ref MERGE_REQUEST_TITLE_PATTERN: grok::Pattern = {
	       let mut grok = grok::Grok::with_patterns();
        grok.insert_definition("TASK", r"[\d]+");
        grok.insert_definition("TITLE", r".+");

        grok.compile("%{TASK} %{TITLE}", false)
            .expect("Error while compiling!")
	 };
}

impl SimpleWebhookService {
    pub fn new(youtrack_service: Arc<RwLock<YoutrackService>>) -> SimpleWebhookService {
        SimpleWebhookService { youtrack_service }
    }

    pub async fn process_web_hook(&mut self, webhook: GitlabHookRequest) -> Result<(), SimpleWebhookServiceError> {
        let result = match webhook {
            GitlabHookRequest::MergeRequest(merge_request_hook) => {
                Ok(self.process_merge_request_hook(merge_request_hook).await)
            }
            GitlabHookRequest::Note(_) =>Err(SimpleWebhookServiceError::NoteHookNotImplemented),
            GitlabHookRequest::Pipeline(_) => Err(SimpleWebhookServiceError::PipelineHookNotImplemented)
        };
        result
    }

    pub async fn process_merge_request_hook(&mut self, merge_request_hook: MergeRequestHook) {
        let merge_request_action = merge_request_hook.object_attributes.action;
        let task_id = {
            match MERGE_REQUEST_TITLE_PATTERN.match_against(merge_request_hook.object_attributes.title.as_ref()) {
                Some(matches) => match matches.get("TASK") {
                    Some(task_id) => Some(format!("PMS-{:?}", task_id)),
                    _ => None
                }
                _ => None
            }
        };
        let state = merge_request_hook.object_attributes.state;

        if let (Some(MergeRequestAction::Merge), Some(task_id), MergeRequestState::Merged) = (merge_request_action, task_id, state) {
            self.youtrack_service.clone().write().await
                .update_status(task_id);
        }

        if let Some(true) = merge_request_hook.object_attributes.merge_when_pipeline_succeeds {
            // set label pipeline in progress
        }
    }
}