mod data;
mod logic;

use crate::{features::helpers::build_client, run_options::issues_sync_run_options};

pub fn run(options: issues_sync_run_options::RunOptions) -> Result<(), Box<dyn std::error::Error>> {
    println!("Starting in issues_sync mode with args: {:?}", &options);

    let client_maker = Box::new(|| build_client(None));

    logic::run_regular(client_maker.as_ref(), options)?;

    println!("End of issues_sync mode reached.");

    Ok(())
}
