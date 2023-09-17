use clap::{Parser, Subcommand};

pub mod issues_sync_run_options;

#[derive(Parser)]
pub struct RunOptions {
    #[clap(subcommand)]
    pub command: RunCommand,
}

#[derive(Subcommand, Clone, Debug)]
pub enum RunCommand {
    ///
    IssuesSync(issues_sync_run_options::RunOptions),
}
