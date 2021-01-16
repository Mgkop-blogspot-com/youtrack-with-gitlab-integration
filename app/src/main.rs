#![feature(box_syntax)]
#[warn(unused_imports)]
#[macro_use]
extern crate lazy_static;

use log::LevelFilter;
use env_logger::WriteStyle;
use youtrack_tools::rest_api::client::YoutrackClient;
use youtrack_tools::rest_api::base::ops::BaseOps;
use youtrack_tools::rest_api::json_models::issue::field::IssueStateType;
use indoc::indoc;
use crate::api::{index, server};

mod api;
mod settings;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::builder()
        .filter(None, LevelFilter::Info)
        .write_style(WriteStyle::Always)
        .init();

    // youtrack_ops().await;
    api::server().await.await
}

async fn youtrack_ops() -> () {
    let token = "perm:c2VnZQ==.NDUtMA==.BwWbdAkDk6Dr6rKnoVD3Rg5PStkMFE".to_string();
    let host = "http://localhost:10100".to_string();
    let client_impl = youtrack_tools::rest_api::client::YoutrackClientImpl::new(host, token).await.unwrap();

    if let Ok(mut issue) = client_impl.issue("SSP-7".to_string()).await{
        // let mut issue = client_impl.issue("SSP-7".to_string()).await;
        issue.set_state(IssueStateType::ToVerify);
        let new_description_text = indoc!("### New description 3
                                Bugaga **from** __rust__
                                [lol](google.com)");
        issue.description = Some(new_description_text.to_string());

        let x = issue.save().await;
    }
}