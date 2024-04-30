use crate::HTTP_CLIENT;
use scraper::{Html, Selector};
use std::collections::HashMap;
use std::process::exit;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
enum TabroomError {
    #[error("Reqwest error")]
    Req(#[from] reqwest::Error),
    #[error("Reqwest middleware error")]
    ReqMiddleware(#[from] reqwest_middleware::Error),
    #[error("Error configuring selector")]
    Selector(#[from] scraper::error::SelectorErrorKind<'static>),
    #[error("Error message: `{0}`")]
    Message(String),
}

#[derive(Debug)]
struct UserInfo {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
}

#[tauri::command]
pub(super) async fn login_tab(email: String, password: String) -> tauri::Result<bool> {
    let mut params = HashMap::new();
    params.insert("username", email);
    params.insert("password", password);

    let req_res = HTTP_CLIENT
        .post("https://www.tabroom.com/user/login/login_save.mhtml")
        .form(&params)
        .send()
        .await;

    debug!("a");

    if req_res.is_err() {
        return Ok(false);
    }

    let res = req_res.unwrap();

    if res.url().path() != "/user/student/index.mhtml" {
        return Ok(false);
    }

    Ok(true)
}

///Gets user info from profile fields
async fn get_user_info() -> Result<UserInfo, TabroomError> {
    let html_text = HTTP_CLIENT
        .get("https://www.tabroom.com/user/login/profile.mhtml")
        .send()
        .await?
        .text()
        .await?;
    let html = Html::parse_document(html_text.as_str());
    let first = html
        .select(&Selector::parse(r#"input[name="first"]"#)?)
        .next()
        .ok_or(TabroomError::Message(
            "Failed to find selection for first name".to_string(),
        ))?
        .attr("value");
    let middle = html
        .select(&Selector::parse(r#"input[name="middle"]"#)?)
        .next()
        .ok_or(TabroomError::Message(
            "Failed to find selection for middle name".to_string(),
        ))?
        .attr("value");
    let last = html
        .select(&Selector::parse(r#"input[name="last"]"#)?)
        .next()
        .ok_or(TabroomError::Message(
            "Failed to find selection for last name".to_string(),
        ))?
        .attr("value");


    Ok(UserInfo {
        first_name: first.map(|f| f.to_string()),
        middle_name: middle.map(|m| m.to_string()),
        last_name: last.map(|l| l.to_string()),
    })
}
