use crate::user_info::Account;
use std::collections::HashMap;
use reqwest::{self, StatusCode};
use std::error::Error;

pub fn create_repository(user_info: &Account, repo_name: &str, private: bool) -> Result<(String, bool), Box<dyn Error>> {
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
