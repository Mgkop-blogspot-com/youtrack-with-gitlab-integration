use actix_web::{get, post, web, Responder, HttpServer, middleware, App, HttpRequest, HttpResponse, error};
use actix_web::dev::Server;
use crate::settings;
use std::sync::Arc;
use tokio::sync::Mutex;
use listenfd::ListenFd;
use tokio::stream::StreamExt;
use gitlab_tools::models::hooks::GitlabHookRequest;
use web_utils::HeaderGetter;
use crate::service::youtrack_service::YoutrackService;

const MAX_SIZE: usize = 262_144;

#[get("/{id}/{name}/index.html")]
pub async fn index(web::Path((id, name)): web::Path<(u32, String)>) -> impl Responder {
    format!("Hello {}! id:{}", name, id)
}


#[post("/api/gitlab/hooks")]
pub async fn merge_request(request: HttpRequest, hook: web::Json<GitlabHookRequest>, youtrack_service: web::Data<YoutrackService>) -> impl Responder {

    // let x_gitlab_event_header = request.headers().get("X-Gitlab-Event").unwrap().to_str().unwrap();
    // let received_token = request.headers().get("x-gitlab-token").unwrap().to_str().unwrap();

    // match (request.get_header("X-Gitlab-Event"), request.get_header("x-gitlab-token")) {
    //     (Some(gitlab_event), Some(gitlab_token)) =>
    //         if settings::get_str("gitlab.token").unwrap() == gitlab_token {
    //             Ok(gitlab_event)
    //         } else {
    //             Err("Received wrong token")
    //         }
    //     (_, _) => Err(r#"Missed requeared headers: "X-Gitlab-Event", "X-Gitlab-Token""#),
    // }.and_then(|ignore|{
    //     hook
    // })

    // .map(|(gitlab_event, gitlab_token)|if settings::get_str("gitlab.token").unwrap() == gitlab_token {
    // gitlab_event
    // }else { Err("Received wrong token") })
    ;
    // Ok(HttpResponse::Ok().json(obj)) // <- send response

    HttpResponse::Ok().body("not implemented")
    // Ok(HttpResponse::Ok().body("not implemented"))
}

pub async fn server() -> Server {
    let mut listenfd = ListenFd::from_env();

    // let api_version = config.get_str("version").unwrap() as ApiVersion;
    //
    // let image_service: Arc<Mutex<ImageService>> = {
    //     let service = new_arango_image_service().await;
    //     Arc::new(Mutex::new(service))
    // };
    //
    // let api_version: Arc<ApiVersion> = {
    //     let version = env!("CARGO_PKG_VERSION").to_string();
    //     let api_version = ApiVersion { version };
    //     Arc::new(api_version)
    // };

    let mut server = HttpServer::new(move || {
        App::new()
            // .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            // .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(index)
        // .service(load_image_base64)
        // .service(load_image_multipart)
        // .service(load_image_link)
        // .service(get_image)
        // .service(get_images)
        // .service(index)
        // .service(ping)
        // .data(api_version.clone())
        // .data(image_service.clone())
        // .service(
        // 	web::resource("/resource2/index.html")
        // 		.wrap(
        // 			middleware::DefaultHeaders::new().header("X-Version-R2", "0.3"),
        // 		)
        // 		.default_service(
        // 			web::route().to(|| HttpResponse::MethodNotAllowed()),
        // 		)
        // 		.route(web::get().to(index_async)),
        // )
        // .service(web::resource("/test1.html").to(|| async { "Test\r\n" }))
    });

    let server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener).unwrap()
    } else {
        let application_port = settings::get_int("app.port").unwrap() as u16;
        let socket = ("0.0.0.0", application_port);
        server.bind(socket).unwrap()
    };
    println!("Selected sockets for web: {:?}", server.addrs());
    server.run()
}
