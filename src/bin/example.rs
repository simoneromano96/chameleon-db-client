use chameleon_db_client::DBClient;
fn main() {
    let mut client = DBClient::new("http://localhost:8529".to_string());
    println!("{:?}", client.is_db_available());
    println!("{:?}", client.authenticate("root", "password"));
    println!("{:?}", client.authenticate("root", "password123"));
}
