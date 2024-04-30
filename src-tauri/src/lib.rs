mod tabroom;

use crate::tabroom::login_tab;
use log::debug;
use once_cell::sync::Lazy;
use reqwest::redirect::Policy;
use reqwest_middleware::ClientBuilder;
use reqwest_retry::policies::ExponentialBackoff;
use reqwest_retry::RetryTransientMiddleware;
use std::collections::HashMap;
use tauri::utils::html::parse;
use tauri_plugin_log::{Target, TargetKind};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

static HTTP_CLIENT: Lazy<reqwest_middleware::ClientWithMiddleware> = Lazy::new(|| {
    let retry_policy = ExponentialBackoff::builder().build_with_max_retries(3);
    let client = reqwest::ClientBuilder::new()
        .user_agent(format!("Presiding Power {}", env!("CARGO_PKG_VERSION")))
        .cookie_store(true)
        .build()
        .expect("Should be able to make client");
    ClientBuilder::new(client)
        // Retry failed requests.
        .with(RetryTransientMiddleware::new_with_policy(retry_policy))
        .build()
});

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir {
                        file_name: Some("log.log".to_string()),
                    }),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![login_tab])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
