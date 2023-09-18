use clap::Parser;

#[derive(Parser, Clone, Debug, Default)]
pub struct RunOptions {
    /// The name of the repository owner (user or organization)
    /// So for a repo with URL https://github.com/some_owner/some_repo, the correct value is some_owner
    #[clap(long)]
    pub repo_owner: String,

    /// The name of the repository
    /// So for a repo with URL https://github.com/some_owner/some_repo, the correct value is some_repo
    #[clap(long)]
    pub repo: String,

    /// The GitHub personal access token to use for authentication
    /// See https://docs.github.com/en/github/authenticating-to-github/creating-a-personal-access-token
    /// for more information
    #[clap(long)]
    pub github_access_token: String,

    /// The directory, in which the data will be stored
    #[clap(long)]
    pub data_dir: std::path::PathBuf,
}
