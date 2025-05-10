#[derive(Debug, Deserialize, FromRow)]
struct User {
    id: uuid::Uuid,
    favoriteColor: String,
    height: bigdecimal::Decimal,
    age: i32,
    job: String,
}
#[derive(Debug, Deserialize, FromRow)]
struct ProductDetails {
    productId: i32,
    description: &str,
    price: i32,
}
#[derive(Debug, Deserialize, FromRow)]
struct Statement {
    orderId: i32,
    itemId: i32,
    quantity: i32,
}
