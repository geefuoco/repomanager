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
    Create(CreateInfo),
    ///Delete a repo
    Delete(DeleteInfo),
}

#[derive(Debug, Args)]
pub struct CreateInfo{
    ///The name of the repository to create 
    pub name: String,
    #[clap(short, long, action)]
    ///The privacy of the repo. Defaults to public
    pub private: bool
}

#[derive(Debug, Args)]
pub struct DeleteInfo {
    ///The name of the repository to delete
    pub name: String
}
