use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RepoManager {
    /// Action to take [create, delete, update]
    #[clap(subcommand)]
    pub action: Action
}

#[derive(Debug, Subcommand)]
pub enum Action {
    ///Create a new repo 
    Create(RepoInfo),
    ///Delete a repo
    Delete(RepoInfo),
}

#[derive(Debug, Args)]
pub struct RepoInfo{
    ///The name of the repository to create
    pub name: String,
    #[clap(short, long, action)]
    ///The privacy of the repo. Defaults to public
    pub private: bool
}
