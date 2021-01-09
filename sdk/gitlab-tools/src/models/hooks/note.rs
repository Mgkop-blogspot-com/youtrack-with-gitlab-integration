use serde::{Serialize, Deserialize};
use crate::models::hooks::base::{Project, LastCommit, Repository, User, Identifier};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteObjectAttributes {
    pub id: Identifier,
    pub note: String,
    pub noteable_type: String,
    pub author_id: Identifier,
    pub created_at: String,
    pub updated_at: String,
    pub project_id: Identifier,
    // pub attachment: ::serde_json::Value,
    // pub line_code: ::serde_json::Value,
    pub commit_id: String,
    pub noteable_id: Identifier,
    pub system: bool,
    // pub st_diff: ::serde_json::Value,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequest {
    pub id: Identifier,
    pub target_branch: String,
    pub source_branch: String,
    pub source_project_id: Identifier,
    pub author_id: Identifier,
    pub assignee_id: Identifier,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub milestone_id: Identifier,
    pub state: String,
    pub merge_status: String,
    pub target_project_id: Identifier,
    pub iid: Identifier,
    pub description: String,
    pub position: Identifier,
    pub source: Project,
    pub target: Project,
    pub last_commit: LastCommit,
    pub work_in_progress: bool,
    pub assignee: User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteHook {
    pub user: User,
    pub project_id: Identifier,
    pub project: Project,
    pub repository: Repository,
    pub object_attributes: NoteObjectAttributes,
    pub merge_request: MergeRequest,
}

#[cfg(test)]
pub mod test {
    use tokio;
    use crate::models::hooks::GitlabHookRequest;
    use env_utils::from_root_file;
    // use gitlab::webhooks::WebHook;

    #[tokio::test]
    async fn parse_example_ok() {
        let data = from_root_file("sdk/gitlab-tools/rest/note/note.example.json").await;
        let result = serde_json::from_slice::<GitlabHookRequest>(&data).unwrap();
    }

    // #[tokio::test]
    // async fn parse_real_ok() {
    //     let data = from_root_file("sdk/gitlab-tools/rest/merge_request/merge_request.real.json").await;
    //     let result = serde_json::from_slice::<GitlabHookRequest>(&data).unwrap();
    // }
}
