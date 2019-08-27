use chameleon_db_client;

fn main() {
    let client = chameleon_db_client::DBClient::new("http://localhost:8529".to_string());
    println!("{:?}", client.db_available());
}
