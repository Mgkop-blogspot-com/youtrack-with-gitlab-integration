use async_trait::async_trait;
use crate::rest_api::project::{Project};
use crate::rest_api::user::User;
use crate::rest_api::issue::{Issue};
use crate::rest_api::base::{NameType, BaseInfo};
use std::sync::{Mutex, Arc};
use hyper::Client;
use hyper::client::HttpConnector;
use tokio::time::Duration;
use crate::rest_api::service::issues::fetch_issue_by_id;
use crate::rest_api::base::client::HttpClient;

pub struct YoutrackClientImpl {
    client: Arc<HttpClient>,
    config: Config,
}

#[derive(Clone, Debug)]
pub struct Config {
    pub host: String,
    pub bearer_token: String,
}

#[async_trait]
trait YoutrackClient: Sync {
    // async fn users(&self) -> Vec<Box<dyn User>>;
    // async fn user(&self, name: NameType) -> Vec<Box<dyn User>>;
    // async fn tasks(&self) -> Vec<Box<dyn Task>>;
    async fn issue(&self, name: NameType) -> Box<Issue>;
    // async fn projects(&self) -> Vec<Box<dyn Project>>;
    // async fn project(&self, name: NameType) -> Box<dyn Project>;
}

impl YoutrackClientImpl {
    pub async fn new(domain: String, bearer_token: String) -> Result<YoutrackClientImpl, ()> {
        let url = "something".parse::<hyper::Uri>().unwrap();

        let mut req = hyper::Request::new(url);
        req.headers_mut().insert(hyper::header::AUTHORIZATION, "Bearer perm:token".parse().unwrap());

        let config = Config { host: domain, bearer_token };
        let youtrack_client = YoutrackClientImpl {
            client: Arc::new(HttpClient::new(config.host.clone())),
            config,
        };
        Ok(youtrack_client)
    }
}

#[async_trait]
impl YoutrackClient for YoutrackClientImpl {
    async fn issue(&self, name: NameType) -> Box<Issue> {
        let http_client = HttpClient::new(self.config.host.clone());
        let origin = fetch_issue_by_id(&http_client, name.clone()).await;
        box Issue::new(http_client, origin)
    }
}