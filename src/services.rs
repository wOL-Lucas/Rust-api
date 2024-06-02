use actix_web::{
    web::{
        scope,
        Json,
        Data,
        ServiceConfig,
        Query
    },
    get,
    post,
    HttpResponse,
    Responder
};

use serde_json::json;
use sqlx;
use crate::{schema::{CreateTaskSchema, FilterOptions}, model::TaskModel, AppState};

#[get("/healthchecker")]
async fn healthchecker() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "ok"
    }))
}

#[post("/task")]
async fn create_task(
    body: Json<CreateTaskSchema>,
    data: Data<AppState> ) -> impl Responder {
    
    println!("task reached");

    match
        sqlx::query_as!(
            TaskModel,
            "INSERT INTO tasks (title, content) VALUES ($1, $2) RETURNING *",
            body.title.to_string(),
            body.content.to_string()
        )
        .fetch_one(&data.db)
        .await {
            Ok(task) => {
                let note_response = json!({
                    "status": "ok",
                    "task": json!({ "task":task })
                });

                return HttpResponse::Ok().json(note_response);
            }
            Err(e) => {
                println!("{:?}", e);
                let error_response = json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                });

                return HttpResponse::InternalServerError().json(error_response);
            }
        }
} 


#[get("/task")]
async fn get_tasks(
    opts: Query<FilterOptions>,
    data: Data<AppState>) -> impl Responder {

    let limit = opts.limit.unwrap_or(10);
    let page = (opts.page.unwrap_or(1) -1) * limit;

    match
        sqlx::query_as!(
            TaskModel,
            "SELECT * FROM tasks ORDER BY id DESC LIMIT $1 OFFSET $2",
            limit as i32,
            page as i32
        )
        .fetch_all(&data.db)
        .await {
            Ok(tasks) => {
                let note_response = json!({
                    "status": "ok",
                    "tasks": tasks
                });

                return HttpResponse::Ok().json(note_response);
            }
            Err(e) => {
                println!("{:?}", e);
                let error_response = json!({
                    "status": "error",
                    "message": format!("{:?}", e)
                });

                return HttpResponse::InternalServerError().json(error_response);
            }
        }
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(healthchecker)
        .service(create_task)
        .service(get_tasks);

    conf.service(scope);
}
