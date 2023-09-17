use clap::Parser;
use features::issues_sync;
use run_options::RunOptions;

mod features;
mod run_options;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = RunOptions::parse();

    match args.command {
        run_options::RunCommand::IssuesSync(options) => issues_sync::run(options),
    }
}
