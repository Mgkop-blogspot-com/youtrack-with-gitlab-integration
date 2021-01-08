#![feature(box_syntax)]

pub mod rest_api;

extern crate async_trait;
extern crate serde;
extern crate log;

#[cfg(test)]
mod tests {
    use hyper::Client;
    use tokio::time::Duration;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_works1() {
        let client: Client<_, hyper::Body> = Client::builder()
            .pool_idle_timeout(Duration::from_secs(30))
            .http2_only(true)
            .build_http();
    }

    #[test]
    fn it_works2() {}
}
