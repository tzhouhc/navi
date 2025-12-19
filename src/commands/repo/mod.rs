use crate::commands;
use crate::prelude::*;
use clap::{Args, Subcommand};

pub mod add;
pub mod browse;

#[derive(Debug, Clone, Subcommand)]
pub enum RepoCommand {
    /// Imports cheatsheets from a repo
    Add {
        /// A URI to a git repository containing .cheat files ("user/repo" will download cheats from github.com/user/repo)
        uri: String,
        /// Import all cheatsheets from repo without prompting
        #[clap(short = 'a', long, visible_short_alias = 'y', visible_alias = "yes")]
        all: bool,
    },
    /// Browses for featured cheatsheet repos
    Browse {
        /// Import all cheatsheets from selected repo without prompting
        #[clap(short = 'a', long, visible_short_alias = 'y', visible_alias = "yes")]
        all: bool,
    },
}

#[derive(Debug, Clone, Args)]
pub struct Input {
    #[clap(subcommand)]
    pub cmd: RepoCommand,
}

impl Runnable for Input {
    fn run(&self) -> Result<()> {
        match &self.cmd {
            RepoCommand::Add { uri, all } => {
                add::main(uri.clone(), *all)
                    .with_context(|| format!("Failed to import cheatsheets from `{uri}`"))?;
                commands::core::main()
            }
            RepoCommand::Browse { all } => {
                let repo = browse::main().context("Failed to browse featured cheatsheets")?;
                add::main(repo.clone(), *all)
                    .with_context(|| format!("Failed to import cheatsheets from `{repo}`"))?;
                commands::core::main()
            }
        }
    }
}
