use async_trait::async_trait;
use std::rc::Rc;

pub type Ideantifier = String;
pub type NameType = String;

#[async_trait]
pub trait BaseInfo {
    async fn name(&self) -> NameType;
    async fn id(&self) -> Ideantifier;
}

pub mod ops {
    use async_trait::async_trait;

    #[async_trait]
    pub trait BaseOps {
        async fn update(&mut self) -> &mut Self;
        // async fn delete(&self) -> Self;
        async fn save(&mut self) -> Self;
    }
}

pub mod client {
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use hyper::{Client, Uri, Response, Body};
    use hyper::client::{HttpConnector, ResponseFuture};
    use std::ops::Deref;
    use serde::export::fmt::Debug;
    use serde::export::Formatter;
    use std::fmt;
    use std::time::Instant;

    pub type HyperClient = Client<HttpConnector, hyper::Body>;

    pub struct HttpClient {
        inner: Arc<HyperClient>,
        created_at: Instant,
        host: String,
    }

    impl HttpClient {
        pub fn new(host: String) -> Self {
            // let client: Client<_, hyper::Body> = Client::builder()
            //     .pool_idle_timeout(Duration::from_secs(30))
            //     .http2_only(true)
            //     .build_http();


            let default_client = Default::default();
            let inner = Arc::new(default_client);
            HttpClient { inner, created_at: Instant::now(), host }
        }

        pub fn refresh_client(&mut self) -> &Self {
            *Arc::make_mut(&mut self.inner) = Default::default();
            self.created_at = Instant::now();
            self
        }

        pub async fn get_uri(&self, uri: Uri) -> hyper::Result<Response<Body>> {
            let pq = uri
                .path_and_query()
                .unwrap();

            let uri = hyper::Uri::builder()
                .scheme(self.host.clone().as_str())
                .path_and_query(pq.clone()).build()
                .unwrap();

            self.inner.get(uri).await
        }
    }

    impl Deref for HttpClient {
        type Target = HyperClient;

        fn deref(&self) -> &Self::Target {
            &*self.inner
        }
    }

    impl Debug for HttpClient {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("HttpClient")
                .field("hyper", &self.inner)
                .field("created at", &self.created_at)
                .finish()
        }
    }
}

pub mod wrap {
    use crate::rest_api::base::client::HttpClient;
    use std::rc::Rc;
    use std::ops::{Deref, DerefMut};
    use std::sync::Arc;

    #[derive(Debug)]
    pub struct ActiveRecordWrap<DTO> {
        pub origin: Arc<DTO>,
        inner: Arc<DTO>,
        pub http_client: HttpClient,
    }

    impl<DTO> ActiveRecordWrap<DTO> {
        pub fn new(http_client: HttpClient, origin: DTO) -> Self {
            let origin = Arc::new(origin);
            let inner = origin.clone();
            Self { origin, inner, http_client }
        }

        pub fn refresh(&mut self, new_origin: DTO) -> &Self {
            self.origin = Arc::new(new_origin);
            self.inner = self.origin.clone();
            self
        }
    }

    impl<DTO> Deref for ActiveRecordWrap<DTO> {
        type Target = DTO;

        fn deref(&self) -> &Self::Target {
            &*self.origin
        }
    }

    impl<DTO> DerefMut for ActiveRecordWrap<DTO>
        where DTO: Clone {
        fn deref_mut(&mut self) -> &mut Self::Target {
            Arc::make_mut(&mut self.inner)
        }
    }
}
