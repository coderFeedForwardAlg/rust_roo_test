#[derive(Debug, Deserialize, FromRow)]
struct Users {
    user_uuid: uuid::Uuid,
    name: String,
    email: String,
}


async fn add_stuff(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<Users>,
) -> Json<Value> {
    let query = "INSERT INTO users (user_uuid, name, email) VALUES ($1, $2, $3)";
    sqlx::query(query)
    	.bind(payload.user_uuid)
	.bind(payload.name)
	.bind(payload.email)
        .execute(&pool)
        .await;
        Json(json!({"res": "sucsess"}))
}


async fn get_stuff(
    extract::State(pool): extract::State<PgPool>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = "SELECT * FROM users";
    let q = sqlx::query_as::<_, Users>(query);

    let elemints: Vec<Users> = q.fetch_all(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
        json!({
    	"user_uuid": elemint.user_uuid, 
	"name": elemint.name, 
	"email": elemint.email, 

        })
    
    }).collect();

    Ok(Json(json!({ "payload": res_json })))
}
#[derive(Debug, Deserialize, FromRow)]
struct Runs {
    run_uuid: uuid::Uuid,
    user_uuid: uuid::Uuid,
    distance: f64,
    time: PgInterval,
    start_time: chrono::DateTime<Utc>,
    FOREIGN: String,
}


async fn add_stuff(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<Runs>,
) -> Json<Value> {
    let query = "INSERT INTO runs (run_uuid, user_uuid, distance, time, start_time, FOREIGN) VALUES ($1, $2, $3, $4, $5, $6)";
    sqlx::query(query)
    	.bind(payload.run_uuid)
	.bind(payload.user_uuid)
	.bind(payload.distance)
	.bind(payload.time)
	.bind(payload.start_time)
	.bind(payload.FOREIGN)
        .execute(&pool)
        .await;
        Json(json!({"res": "sucsess"}))
}


async fn get_stuff(
    extract::State(pool): extract::State<PgPool>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = "SELECT * FROM runs";
    let q = sqlx::query_as::<_, Runs>(query);

    let elemints: Vec<Runs> = q.fetch_all(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
        json!({
    	"run_uuid": elemint.run_uuid, 
	"user_uuid": elemint.user_uuid, 
	"distance": elemint.distance, 
	"time": elemint.time, 
	"start_time": elemint.start_time, 
	"FOREIGN": elemint.FOREIGN, 

        })
    
    }).collect();

    Ok(Json(json!({ "payload": res_json })))
}
