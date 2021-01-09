use serde::{self, Serialize, Deserialize};
use crate::models::hooks::base::{LastCommit, Project, Repository, User, Identifier, MilestoneId, AvatarUrl};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MRObjectAttributes {
    pub id: Identifier,
    pub target_branch: String,
    pub source_branch: String,
    pub source_project_id: Identifier,
    pub author_id: Identifier,
    pub assignee_id: Option<Identifier>,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub milestone_id: MilestoneId,
    pub state: String,
    pub merge_status: String,
    pub target_project_id: Identifier,
    pub iid: Identifier,
    pub description: String,
    pub source: Project,
    pub target: Project,
    pub last_commit: LastCommit,
    pub work_in_progress: bool,
    pub url: String,
    pub action: Option<String>,
    pub assignee: Option<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    pub id: Identifier,
    pub title: String,
    pub color: String,
    pub project_id: Identifier,
    pub created_at: String,
    pub updated_at: String,
    pub template: bool,
    pub description: String,
    pub group_id: Identifier,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Changes {
    pub updated_by_id: Option<UpdatedById>,
    pub updated_at: Option<UpdatedAt>,
    pub labels: Option<Labels>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatedById {
    pub previous: Option<Identifier>,
    pub current: Identifier,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdatedAt {
    pub previous: Option<String>,
    pub current: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Labels {
    pub previous: Vec<Label>,
    pub current: Vec<Label>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MergeRequestHook {
    pub user: User,
    pub project: Project,
    pub repository: Repository,
    pub object_attributes: MRObjectAttributes,
    pub labels: Vec<Label>,
    pub changes: Option<Changes>,
}

#[cfg(test)]
pub mod test {
    use tokio;
    use crate::models::hooks::GitlabHookRequest;
    use env_utils::from_root_file;
    // use gitlab::webhooks::WebHook;

    #[tokio::test]
    async fn parse_example_ok() {
        let data = from_root_file("sdk/gitlab-tools/rest/merge_request/merge_request.example.json").await;
        let result = serde_json::from_slice::<GitlabHookRequest>(&data).unwrap();
    }

    #[tokio::test]
    async fn parse_real_ok() {
        let data = from_root_file("sdk/gitlab-tools/rest/merge_request/merge_request.real.json").await;
        let result = serde_json::from_slice::<GitlabHookRequest>(&data).unwrap();
    }
}
