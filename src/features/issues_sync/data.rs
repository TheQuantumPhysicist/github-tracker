use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, std::hash::Hash)]
pub struct GithubIssueOutline {
    pub title: String,
    pub number: u64,
    pub url: String,
    pub assignees: Vec<GithubUser>,
    pub milestone: Option<GithubMilestone>,
    pub state: String,
    pub body: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub closed_by: Option<String>,
    pub state_reason: Option<String>,
    #[serde(rename = "comments")]
    pub comments_count: u64,
    pub comments_url: String,
    pub events_url: String,
    pub html_url: String,
    pub labels_url: String,
    pub repository_url: String,
    pub id: u64,
    pub locked: bool,
    pub user: GithubUser,
    pub labels: Vec<GithubLabel>,
    pub reactions: GithubReactions,
    pub timeline_url: String,
    pub performed_via_github_app: Option<String>,
    #[serde(rename = "pull_request")]
    pub pull_request: Option<GithubPullRequest>, // Only available if pull-request
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, std::hash::Hash)]
pub struct GithubPullRequest {
    pub url: String,
    pub html_url: String,
    pub diff_url: String,
    pub patch_url: String,
    pub merged_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, std::hash::Hash)]
pub struct GithubUser {
    pub login: String,
    pub id: u64,
    pub node_id: String,
    pub avatar_url: String,
    pub gravatar_id: String,
    pub url: String,
    pub html_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    pub starred_url: String,
    pub subscriptions_url: String,
    pub organizations_url: String,
    pub repos_url: String,
    pub events_url: String,
    pub received_events_url: String,
    #[serde(rename = "type")]
    pub user_type: String,
    pub site_admin: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, std::hash::Hash)]
pub struct GithubMilestone {
    pub url: String,
    pub html_url: String,
    pub labels_url: String,
    pub id: u64,
    pub node_id: String,
    pub number: u64,
    pub state: String,
    pub title: String,
    pub description: Option<String>,
    pub creator: GithubUser,
    pub open_issues: u64,
    pub closed_issues: u64,
    pub created_at: String,
    pub updated_at: String,
    pub closed_at: Option<String>,
    pub due_on: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, std::hash::Hash)]
pub struct GithubIssueComment {
    pub url: String,
    pub html_url: String,
    pub issue_url: String,
    pub id: u64,
    pub node_id: String,
    pub user: GithubUser,
    pub created_at: String,
    pub updated_at: String,
    pub author_association: String,
    pub body: String,
    pub reactions: GithubReactions,
    pub performed_via_github_app: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, std::hash::Hash)]
pub struct GithubLabel {
    pub id: u64,
    pub node_id: String,
    pub url: String,
    pub name: String,
    pub color: String,
    pub default: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, std::hash::Hash)]
pub struct GithubReactions {
    pub url: String,
    pub total_count: u64,
    #[serde(rename = "+1")]
    pub plus_one: u64,
    #[serde(rename = "-1")]
    pub minus_one: u64,
    pub laugh: u64,
    pub hooray: u64,
    pub confused: u64,
    pub heart: u64,
    pub rocket: u64,
    pub eyes: u64,
}
