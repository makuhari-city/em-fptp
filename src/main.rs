use actix_cors::Cors;
use actix_web::{get, middleware, post, web, App, HttpServer, Responder};
use em_fptp::calculate;
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;
use vote::rpc::{JsonRPCRequest, JsonRPCResponse};

type ModuleMap = Mutex<HashMap<String, String>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=trace,actix_redis=trace,vote=debug");
    env_logger::init();

    let modules: web::Data<ModuleMap> = web::Data::new(Mutex::new(HashMap::new()));

    HttpServer::new(move || {
        // TODO: change this
        let cors = Cors::permissive();

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(modules.clone())
            .service(web::scope("/fptp").service(hello).service(calculate_fptp))
    })
    .bind("0.0.0.0:8101")?
    .run()
    .await
}

#[get("/hello/")]
async fn hello() -> impl Responder {
    "hello"
}

#[post("/rpc/")]
async fn calculate_fptp(rpc: web::Json<JsonRPCRequest>) -> impl Responder {
    let rpc = rpc.into_inner();
    let mut response = JsonRPCResponse::new(&rpc.id());
    let result = calculate(&rpc.vote_info()).await;
    response.result(&json!(result));
    web::Json(response)
}
