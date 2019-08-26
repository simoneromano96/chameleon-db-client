use chameleon_db_client;

fn main() {
    let client = chameleon_db_client::DBClient::new("http://jsonplaceholder.typicode.com".to_string());
    client.get("/todos/1");
    client.get("/comments/2");
}
