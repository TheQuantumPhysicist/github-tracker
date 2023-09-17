use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::run_options::issues_sync_run_options;

use super::data;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
}

pub fn run_regular(
    client_maker: &dyn Fn() -> reqwest::blocking::Client,
    options: issues_sync_run_options::RunOptions,
) -> Result<(), Error> {
    let owner = options.repo_owner;
    let repo = options.repo;
    let access_token = options.github_access_token;
    let client = client_maker();

    let version = env!("CARGO_PKG_VERSION");
    let agent = format!("GithubTracker/{}", version);

    let token_value = format!("token {}", access_token);

    for page in 1..=u32::MAX {
        let issues_url =
            format!("https://api.github.com/repos/{owner}/{repo}/issues?page={page}&state=all");

        let response = client
            .get(issues_url)
            .header(USER_AGENT, &agent)
            .header(AUTHORIZATION, &token_value)
            .send()?;

        if !response.status().is_success() {
            return Err(Error::Reqwest(response.error_for_status().unwrap_err()));
        }

        let issues = response.json::<Vec<data::GithubIssueOutline>>()?;
        if issues.is_empty() {
            println!("Done. No more issues found on page {page}.");
            break;
        }
        println!("Got {} issues on page {page}", issues.len());
    }

    Ok(())
}
