use crate::user_info::{Account, get_github_info};
use crate::input::read_input;
use std::path::Path;
use std::collections::HashMap;
use reqwest::{self, StatusCode};
use std::error::Error;
use std::process::Command;


pub fn create_repository(repo: &str, private: &bool) {
    println!("Creating repository: {}", &repo);
    let info = get_github_info().unwrap();
    let (result, success) = create_remote_repository(&info, &repo, private).unwrap();
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
                    println!("Successfully added repo");
                },
                _ => ()
            }
        } else {
            println!("Not currently in a git repository.")
        }
        format!("Finished. View at https://github.com/{}/{}", &info.user(), &repo);
    }
}

pub fn delete_repository(repo: &str) {
    let info = get_github_info().unwrap();
    let (result, _) = delete_remote_repository(&info, repo).unwrap();
    println!("{}", result);
}

fn create_remote_repository(user_info: &Account, repo_name: &str, private: &bool) -> Result<(String, bool), Box<dyn Error>> {
    let api_endpoint = "https://api.github.com/user/repos";
    let header = "application/vnd.github+json";
    let user_agent = "repomaker/0.1.0";
    let mut map = HashMap::new();
    map.insert("name", repo_name);
    match private {
        true => map.insert("private", "true"),
        _ => map.insert("private", "false")

    };
    let client = reqwest::blocking::Client::new();

    let res = client.post(api_endpoint)
        .json(&map)
        .header(reqwest::header::ACCEPT, header)
        .header(reqwest::header::USER_AGENT, user_agent)
        .basic_auth(user_info.user(), Some(user_info.password()))
        .send()?;

    match res.status() {
        StatusCode::CREATED => Ok((String::from("Repository has been successfully created."), true)),
        StatusCode::UNPROCESSABLE_ENTITY => Ok((String::from("Error: Repo already exists. Please try a different name."), false)),
        StatusCode::UNAUTHORIZED => Ok((String::from("Error: Please make sure your credentials are correct."), false)),
        _ => Ok((String::from("An unknown error has occured."), false))
    }
}

fn delete_remote_repository(user_info: &Account, repo_name: &str) -> Result<(String, bool), Box<dyn Error>> {
    let api_endpoint = format!("https://api.github.com/repos/{}/{}", user_info.user(), repo_name);
    let header = "application/vnd.github+json";
    let auth = format!("token {}", user_info.password());
    let user_agent = "repomaker/0.1.0";
    let client = reqwest::blocking::Client::new();
    let res = client.delete(api_endpoint)
        .header(reqwest::header::ACCEPT, header)
        .header(reqwest::header::AUTHORIZATION, auth)
        .header(reqwest::header::USER_AGENT, user_agent)
        .send()?;
    match res.status() {
        StatusCode::NO_CONTENT => Ok((String::from("Repository has been successfully deleted."), true)),
        StatusCode::NOT_FOUND => Ok((String::from("Could not find the provided repo"), false)),
        StatusCode::FORBIDDEN=> Ok((String::from("Not authorized to delete this repo"), false)),
        _ => Ok((String::from("An unknown error has occured"), false))

    }

}
