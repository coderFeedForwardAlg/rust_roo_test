use axum::{
    extract::{self, Extension, Path},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::{json, Value};
use sqlx::PgPool;
use sqlx::{postgres::PgPoolOptions, prelude::FromRow};
use std::env;
use std::net::SocketAddr;
use std::result::Result;
use std::sync::Arc;
use axum::http::StatusCode;





#[derive(Debug, Deserialize, FromRow)]
struct User {
    id: uuid::Uuid,
    favorite_color: String,
    height: i32,
    age: i32,
    job: String,
}


// async fn add_stuff(
//     extract::State(pool): extract::State<PgPool>,
//     Json(payload): Json<User>,
// ) -> Json<Value> {
//     let query = "INSERT INTO user (id, favorite_color, height, age, job) VALUES ($1, $2, $3, $4, $5)";
//     sqlx::query(query)
//     	.bind(payload.id)
// 	.bind(payload.favorite_color)
// 	.bind(payload.height)
// 	.bind(payload.age)
// 	.bind(payload.job)
//         .execute(&pool)
//         .await;
//         Json(json!({"res": "sucsess"}))
// }


// async fn get_stuff(
//     extract::State(pool): extract::State<PgPool>,
// ) -> Result<Json<Value>, (StatusCode, String)> {
//     let query = "SELECT * FROM user";
//     let q = sqlx::query_as::<_, User>(query);

//     let elemints: Vec<User> = q.fetch_all(&pool).await.map_err(|e| {
//         (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
//     })?;

//     let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
//         json!({
//     	"id": elemint.id, 
// 	"favorite_color": elemint.favorite_color, 
// 	"height": elemint.height, 
// 	"age": elemint.age, 
// 	"job": elemint.job, 

//         })
    
//     }).collect();

//     Ok(Json(json!({ "payload": res_json })))
// }






#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //let db_url = env::var("DB_URL").expect("missing db url");
    let db_url = "postgres://dbuser:p@localhost:1111/work";
    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(db_url)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    let app = Router::new();
        // .route("/add_info", post(add_stuff))
        // .route("/get_info", get(get_stuff))
        // .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8081").await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
