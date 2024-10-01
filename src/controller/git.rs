use reqwest::{self, StatusCode};
use std::collections::HashMap;
use axum::{
    response::IntoResponse,
    Json};
use serde::{Serialize, Deserialize};
use crate::util::members;

#[derive(Serialize, Deserialize, Debug)]
pub struct Git {
    action: String,
    pull_request: PullRequest,
    requested_reviewer: Option<RequestedReviewer>
}
#[derive(Serialize, Deserialize, Debug)]
struct PullRequest {
    url: String,
    user: User
}

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
struct RequestedReviewer {
    name: String
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct Member {
    gitId: String,
    slackId: String
}


pub async fn root_handler(body: Json<Git>) -> impl IntoResponse {
    let members = members::read();

    if body.action == "review_requested" {
        let name =  match &body.requested_reviewer {
            Some(reviewer) => {
                &reviewer.name
            },
            None => {
                &"".to_string()
            }
        };
        println!("{:?}", name);
        let mention = members.into_iter().find(|member| member.gitId == *name).unwrap().slackId;
        println!("{:?}", mention);
        let _ = send_slack(&format!("{:?} がプルリク見てって言ってるよ 見てあげなよ\n{:?}", body.pull_request.user.name, body.pull_request.url), &mention).await;
        
    }
    if body.action == "opened" {
        let _ = send_slack(&format!("{:?}がプルリク作ったよ みてあげてね\n{:?}", body.pull_request.user.name, body.pull_request.url), "U07AHBRP38C").await;
    }
    {
        StatusCode::OK
    };
}

async fn send_slack(text: &str, user: &str)-> Result<(), Box<dyn std::error::Error>> {
     let url = "https://hooks.slack.com/triggers/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";
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("mention", user);
    map.insert("text", text);
    map.insert("channel", "C07PMLQNTCK");
    let client = reqwest::Client::new();
    let result = client.post(url)
        .json(&map)
        .send()
        .await?;
    println!("{:?}", result);
    Ok(())
}
