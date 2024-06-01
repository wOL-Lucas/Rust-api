use actix_web::{
    web::{
        scope,
        ServiceConfig,
    },
    get,
    HttpResponse,
    Responder
};

use serde_json::json;


#[get("/healthchecker")]
async fn healthchecker() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok"
    }))
}


pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(healthchecker);
    conf.service(scope);
}
