use tokio::sync::RwLock;
use std::sync::Arc;

pub mod youtrack_service;
pub mod webhook_service;
pub mod gitlab_service;
pub mod grok_service;

pub type Service<T> = Arc<RwLock<T>>;

pub fn new_service<T>(value: T) -> Service<T> where T: Sized {
    let lock = RwLock::new(value);
    Arc::new(lock)
}