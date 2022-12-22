#![allow(dead_code)]

use tokio::task;
use futures::future::join_all;

use reqwest::ClientBuilder;
use reqwest::Result;
use serde::Deserialize;

use std::time::Duration;

use colored::Colorize;

use crate::logger;

/////////////////////////////
// User struct declaration //
/////////////////////////////
#[derive(Deserialize, Debug)]
struct User {
    login: String,
    name: String,
    twitter_username: String,
}

async fn github_test(user: &str) -> Result<()> {
    let url = format!("https://api.github.com/users/{}", &user,);
    let mut _url = reqwest::Url::parse(&url).unwrap();

    let timeout = Duration::new(5, 0);
    let client = ClientBuilder::new()
        .timeout(timeout)
        .user_agent("ferris-tester".to_string())
        .build()?;
    let resp = client.get(_url).send().await?;

    // If response status == 200, Deserialize response body into User struct
    //   - Print out new User data
    if resp.status().is_success() {
        let user: User = resp.json().await?;
        println!();
        logger::success(format!("{:?}", user).as_str());
    } else {
        println!();
        logger::warn(format!("{} {}", &user.purple().bold(), "does not exist!").as_str());
    }

    Ok(())
}

#[tokio::main]
pub async fn start() -> Result<()> {

    // Create an example array of names to use for testing
    let names = ["dead_wood","eazy_e","dedSyn4ps3","ferris_wheel"];

    // Create futures and add to a collection
    //   - One `vector` for async requests
    let mut request_futures = vec![];

    // Loop over `names` and create new futures for each
    for name in names {
        let _future1 = task::spawn(github_test(name));
        request_futures.push(_future1);
    }

    // Join our futures
    let _ = join_all(request_futures).await;

    Ok(())
}