use chameleon_db_client::DBClient;
fn main() {
    let mut client = DBClient::new("http://167.86.100.118:8529".to_string());
    println!("{:?}", client.is_db_available());
    println!("{:?}", client.authenticate("root", "password123"));
    println!("{:?}", client.authenticate("root", "password"));
    let databases = client.get_all_databases().unwrap();
    println!("{:?}, {:?}", databases, databases[0]);
    client.select_database("test");
    let collections = client.get_all_collections().unwrap();
    println!("{:?}", collections);
}
