mod input;
mod user_info;
mod actions;
use input::read_input;
use user_info::get_github_info;
use actions::create_repository;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("Please enter a repo name");
    let repo = loop {
        let name = read_input().unwrap().trim().to_owned();
        if !name.is_empty() {
            break name;
        }
        println!("Cannot leave name empty. Please try again.")
    };
    println!("Creating repository: {}", &repo);
    let info = get_github_info().unwrap();
    let (result, success) = create_repository(&info, &repo, true).unwrap();
    println!("{}", result);
    if success {
        let git = Path::new("./.git").is_dir();
        if git {
            println!("Would you like to connect your current folder to the new repo ? [y/n]");
            let answer = loop {
                let input = read_input().unwrap().trim().to_lowercase().to_owned();
                if input == "n" || input == "y" {
                    break input;
                }
            };
            match answer.as_str() {
                "y" => {
                    Command::new("/bin/bash")
                            .arg("-c")
                            .arg(format!("git remote add origin git@github.com:{}/{}.git", &info.user(), &repo))
                            .output()
                            .expect("An error occured while trying to push to github.");
                },
                _ => ()
            }
            println!("Successfully added repo");
        } else {
            println!("Not currently in a git repository.")
        }
        format!("Finished. View at https://github.com/{}/{}", &info.user(), &repo);
    }
}



