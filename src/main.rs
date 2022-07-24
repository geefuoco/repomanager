mod input;
mod user_info;
mod actions;
mod args;

use args::{RepoManager, Action};
use actions::create_repository;
use clap::Parser;

fn main() {

    let args = RepoManager::parse();

    match &args.action {
        Action::Create(info) => create_repository(&info.name, &info.private),
        _ => println!("Action not yet implemented")
    }

}




