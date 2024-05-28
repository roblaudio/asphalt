use anyhow::Context;
use clap::Parser;
use cli::{Cli, Commands};
use commands::{init::init, list::list, sync::sync};
use dotenv::dotenv;
pub use lockfile::{FileEntry, LockFile};

pub mod cli;
mod commands;
pub mod lockfile;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let args = Cli::parse();

    let existing_lockfile = LockFile::read().await.context("Failed to read lockfile")?;

    match args.command {
        Commands::Sync(sync_args) => sync(sync_args, existing_lockfile)
            .await
            .context("Failed to sync"),
        Commands::List => list(existing_lockfile).await.context("Failed to list"),
        Commands::Init => init().await.context("Failed to initialize"),
    }
}
