
#[derive(Debug, Deserialize, FromRow)]
struct User {
    id: uuid::Uuid,
    favorite_color: String,
    height: i32,
    age: i32,
    job: String,
}


async fn add_stuff(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<User>,
) -> Json<Value> {
    let query = "INSERT INTO user (id, favorite_color, height, age, job) VALUES ($1, $2, $3, $4, $5)";
    sqlx::query(query)
    	.bind(payload.id)
	.bind(payload.favorite_color)
	.bind(payload.height)
	.bind(payload.age)
	.bind(payload.job)
        .execute(&pool)
        .await;
        Json(json!({"res": "sucsess"}))
}


async fn get_stuff(
    extract::State(pool): extract::State<PgPool>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = "SELECT * FROM user";
    let q = sqlx::query_as::<_, User>(query);

    let elemints: Vec<User> = q.fetch_all(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
        json!({
    	"id": elemint.id, 
	"favorite_color": elemint.favorite_color, 
	"height": elemint.height, 
	"age": elemint.age, 
	"job": elemint.job, 

        })
    
    }).collect();

    Ok(Json(json!({ "payload": res_json })))
}
#[derive(Debug, Deserialize, FromRow)]
struct ProductDetails {
    product_id: i32,
    description: String,
    price: i32,
}


async fn add_stuff(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<ProductDetails>,
) -> Json<Value> {
    let query = "INSERT INTO product_details (product_id, description, price) VALUES ($1, $2, $3)";
    sqlx::query(query)
    	.bind(payload.product_id)
	.bind(payload.description)
	.bind(payload.price)
        .execute(&pool)
        .await;
        Json(json!({"res": "sucsess"}))
}


async fn get_stuff(
    extract::State(pool): extract::State<PgPool>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = "SELECT * FROM product_details";
    let q = sqlx::query_as::<_, ProductDetails>(query);

    let elemints: Vec<ProductDetails> = q.fetch_all(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
        json!({
    	"product_id": elemint.product_id, 
	"description": elemint.description, 
	"price": elemint.price, 

        })
    
    }).collect();

    Ok(Json(json!({ "payload": res_json })))
}
#[derive(Debug, Deserialize, FromRow)]
struct OrderItems {
    order_id: i32,
    item_id: i32,
    quantity: i32,
}


async fn add_stuff(
    extract::State(pool): extract::State<PgPool>,
    Json(payload): Json<OrderItems>,
) -> Json<Value> {
    let query = "INSERT INTO order_items (order_id, item_id, quantity) VALUES ($1, $2, $3)";
    sqlx::query(query)
    	.bind(payload.order_id)
	.bind(payload.item_id)
	.bind(payload.quantity)
        .execute(&pool)
        .await;
        Json(json!({"res": "sucsess"}))
}


async fn get_stuff(
    extract::State(pool): extract::State<PgPool>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let query = "SELECT * FROM order_items";
    let q = sqlx::query_as::<_, OrderItems>(query);

    let elemints: Vec<OrderItems> = q.fetch_all(&pool).await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
    })?;

    let res_json: Vec<Value> = elemints.into_iter().map(|elemint| {
        json!({
    	"order_id": elemint.order_id, 
	"item_id": elemint.item_id, 
	"quantity": elemint.quantity, 

        })
    
    }).collect();

    Ok(Json(json!({ "payload": res_json })))
}
