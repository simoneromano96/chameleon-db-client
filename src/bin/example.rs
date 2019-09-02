use chameleon_db_client::db_client::{Collection, Database};
use chameleon_db_client::DBClient;

fn main() {
    let mut client = DBClient::new("http://localhost:8529");
    println!("{:?}", client.is_db_available());
    println!("{:?}", client.authenticate("root", "password123"));
    println!("{:?}", client.authenticate("root", "password"));
    // let databases = client.get_all_databases().unwrap();
    // println!("{:?}, {:?}", databases, databases[0]);
    client.select_database("test");
    let collections: Vec<Collection> = client.get_all_collections().unwrap();
    println!("{:?}", collections);
    // Must be rewritten with the new structure
    // let collection: Collection = client.get_collection("test1").unwrap();
    // println!("{:?}", collection);
    // let new_collection: Collection = client.post_collection("test3").unwrap();
    // println!("{:?}", new_collection);
    let mut new_database: Database = Database::new_local("cicciopernacchio2");
    println!("{:?}", new_database);
    // let created: bool = new_database.create_database(&client).unwrap();
    // println!("{:?}", created);
    let db_info: Database = new_database.get_database_info(&client).unwrap();
    println!("{:?}", db_info);
}
