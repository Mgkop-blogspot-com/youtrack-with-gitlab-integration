use async_trait::async_trait;
use crate::rest_api::project::{Project};
use crate::rest_api::user::User;
use crate::rest_api::issue::{Issue, IssueImpl};
use crate::rest_api::base::{NameType, BaseInfo};
use std::sync::{Mutex, Arc};
use hyper::Client;
use hyper::client::HttpConnector;
use tokio::time::Duration;

pub struct YoutrackClientImpl {
    url: String,
    token: String,
    client: Arc<Mutex<Client<HttpConnector, hyper::Body>>>,
}

#[async_trait]
trait YoutrackClient: Sync {
    // async fn users(&self) -> Vec<Box<dyn User>>;
    // async fn user(&self, name: NameType) -> Vec<Box<dyn User>>;
    // async fn tasks(&self) -> Vec<Box<dyn Task>>;
    async fn task(&self, name: NameType) -> Vec<Box<IssueImpl>>;
    // async fn projects(&self) -> Vec<Box<dyn Project>>;
    // async fn project(&self, name: NameType) -> Box<dyn Project>;
}

impl YoutrackClientImpl {
    pub async fn new(domain: String, bearer_token: String) -> Result<YoutrackClientImpl, ()> {
        let url = "something".parse::<hyper::Uri>().unwrap();

        let mut req = hyper::Request::new(url);
        req.headers_mut().insert(hyper::header::AUTHORIZATION, "Bearer perm:token".parse().unwrap());

        let client: Client<_, hyper::Body> = Client::builder()
            .pool_idle_timeout(Duration::from_secs(30))
            .http2_only(true)
            .build_http();

        let youtrack_client = YoutrackClientImpl {
            url: "".to_string(),
            token: "".to_string(),
            client: Arc::new(Mutex::new(Default::default())),
        };
        Ok(youtrack_client)
    }
}

#[async_trait]
impl YoutrackClient for YoutrackClientImpl {
    async fn task(&self, name: NameType) -> Vec<Box<IssueImpl>> {
        unimplemented!()
    }
}