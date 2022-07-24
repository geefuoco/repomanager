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
    ///Update a repo 
    Update(UpdateInfo)
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


#[derive(Debug, Args)]
pub struct UpdateInfo {
    ///The name of the repo
    pub name: String,
    ///Privacy of the repo. Defaults to false
    pub private: bool

}
