use reqwest::header::{AUTHORIZATION, USER_AGENT};

use crate::run_options::issues_sync_run_options;

use super::data::{self, GithubIssueOutline};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Issue outline file read error: {0}")]
    IssueOutlineFileRead(std::io::Error),
    #[error("Issue outline file parsing error: {0}")]
    IssueOutlineFileParse(serde_json::Error),
    #[error("Comments file creation failed for issue {0} and comments page {1}: {2}")]
    CommentsFileCreationFailed(std::io::Error, u64, u32),
    #[error("Comment file writing failed for issue {0} and comments page {1}: {2}")]
    CommentFileWriteFailed(serde_json::Error, u64, u32),
    #[error("Issue outline file creation failed for issue {0}: {1}")]
    IssueOutlineFileCreationFailed(std::io::Error, u64),
    #[error("Issue outline file writing failed for issue {0}: {1}")]
    IssueOutlineFileWriteFailed(serde_json::Error, u64),
    #[error("Failed to created data-directory: {0}")]
    DataDirCreationFailed(std::io::Error),
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

    let base_repo_api_url = format!(
        "https://api.github.com/repos/{owner}/{repo}",
        owner = owner,
        repo = repo
    );

    // Create the data directory if it doesn't exist
    std::fs::create_dir_all(&options.data_dir).map_err(Error::DataDirCreationFailed)?;

    for page in 1..=u32::MAX {
        let issues_url = format!("{base_repo_api_url}/issues?page={page}&state=all");

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
        println!("Found {} issues on page {page}", issues.len());

        // Loop over all issues and download all comments for each issue
        for issue_outline in issues {
            // If the issue outline file already exists and is up to date, skip this issue
            if !is_an_update_needed(&options.data_dir, &issue_outline)? {
                continue;
            }

            println!(
                "Issue #{number} needs an update. Downloading all data and comments and writing it.",
                number = issue_outline.number
            );
            write_all_issue_data(
                &options.data_dir,
                &client,
                &base_repo_api_url,
                &agent,
                &token_value,
                issue_outline,
            )?;
        }
    }

    Ok(())
}

fn download_and_write_all_comments_for_issue<P: AsRef<std::path::Path>>(
    data_dir: P,
    client: &reqwest::blocking::Client,
    base_api_repo_url: &str,
    agent: &str,
    token_value: &str,
    issue_number: u64,
) -> Result<(), Error> {
    for page in 1..=u32::MAX {
        let comments_url =
            format!("{base_api_repo_url}/issues/{issue_number}/comments?page={page}");

        let response = client
            .get(comments_url)
            .header(USER_AGENT, agent)
            .header(AUTHORIZATION, token_value)
            .send()?;

        if !response.status().is_success() {
            return Err(Error::Reqwest(response.error_for_status().unwrap_err()));
        }

        let comments_data = response.json::<Vec<data::GithubIssueComment>>()?;
        if comments_data.is_empty() {
            println!("Done. No more comments found on page {page}.");
            break;
        }

        {
            let comments_file_path = issue_number_to_comments_path(&data_dir, issue_number, page);
            let comments_file = std::fs::File::create(&comments_file_path)
                .map_err(|e| Error::CommentsFileCreationFailed(e, issue_number, page))?;
            serde_json::to_writer(comments_file, &comments_data)
                .map_err(|e| Error::CommentFileWriteFailed(e, issue_number, page))?;
        }

        println!("Got {} comments on page {page}", comments_data.len());
    }

    Ok(())
}

fn issue_obj_to_issue_outline_file_path<P: AsRef<std::path::Path>>(
    data_dir: P,
    issue: &GithubIssueOutline,
) -> std::path::PathBuf {
    if issue.pull_request.is_some() {
        data_dir.as_ref().join(format!("pr_{}.json", issue.number))
    } else {
        data_dir
            .as_ref()
            .join(format!("issue_{}.json", issue.number))
    }
}

fn issue_number_to_comments_path<P: AsRef<std::path::Path>>(
    data_dir: P,
    issue_number: u64,
    page_number: u32,
) -> std::path::PathBuf {
    data_dir
        .as_ref()
        .join(format!("issue_{issue_number}_comments-{page_number}.json"))
}

fn is_an_update_needed<P: AsRef<std::path::Path>>(
    data_dir: P,
    latest_issue_data: &data::GithubIssueOutline,
) -> Result<bool, Error> {
    let issue_outline = load_issue_outline_from_file(data_dir, latest_issue_data)?;

    match issue_outline {
        None => Ok(true),
        Some(issue_outline) => Ok(issue_outline != *latest_issue_data),
    }
}

fn load_issue_outline_from_file<P: AsRef<std::path::Path>>(
    data_dir: P,
    issue_from_api: &GithubIssueOutline,
) -> Result<Option<GithubIssueOutline>, Error> {
    let issue_file_path = issue_obj_to_issue_outline_file_path(data_dir, issue_from_api);

    if !issue_file_path.exists() {
        return Ok(None);
    }

    let issue_file = std::fs::File::open(issue_file_path).map_err(Error::IssueOutlineFileRead)?;

    let issue_outline = serde_json::from_reader::<_, GithubIssueOutline>(&issue_file)
        .map_err(Error::IssueOutlineFileParse)?;

    Ok(Some(issue_outline))
}

fn write_all_issue_data<P: AsRef<std::path::Path>>(
    data_dir: P,
    client: &reqwest::blocking::Client,
    base_repo_api_url: &str,
    agent: &str,
    token_value: &str,
    issue_outline: data::GithubIssueOutline,
) -> Result<(), Error> {
    let issue_number = issue_outline.number;
    let issue_file_path = issue_obj_to_issue_outline_file_path(&data_dir, &issue_outline);
    {
        let issue_file = std::fs::File::create(&issue_file_path)
            .map_err(|e| Error::IssueOutlineFileCreationFailed(e, issue_number))?;
        serde_json::to_writer_pretty(issue_file, &issue_outline)
            .map_err(|e| Error::IssueOutlineFileWriteFailed(e, issue_number))?;
    }

    download_and_write_all_comments_for_issue(
        &data_dir,
        &client,
        &base_repo_api_url,
        &agent,
        &token_value,
        issue_number,
    )?;

    Ok(())
}
