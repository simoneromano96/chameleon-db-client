use chameleon_db_client::db_client::{Collection, Database};
use chameleon_db_client::DBClient;

fn main() {
    let mut client = DBClient::new("http://localhost:8529");
    println!("{:?}", client.is_db_available());
    println!("{:?}", client.authenticate("root", "password123"));
    println!("{:?}", client.authenticate("root", "password"));
    // ! These will be changed and must be rewritten !
    // let databases = client.get_all_databases().unwrap();
    // println!("{:?}, {:?}", databases, databases[0]);
    // client.select_database("test");
    // let collections: Vec<Collection> = client.get_all_collections().unwrap();
    // println!("{:?}", collections);
    // let collection: Collection = client.get_collection("test1").unwrap();
    // println!("{:?}", collection);
    // let new_collection: Collection = client.post_collection("test3").unwrap();
    // println!("{:?}", new_collection);
    println!("{:?}", client.databases);
    let mut new_database1: Database = Database::new_local("test123");
    println!("{:?}", new_database1);
    let mut new_database2: Database = Database::new_local("test345");
    println!("{:?}", new_database2);
    let mut new_database3: Database = Database::new_local("test567");
    println!("{:?}", new_database3);

    println!("{:?}", client.databases);
    let created: bool = new_database1.create_database(&mut client).unwrap();
    println!("{:?}", created);
    println!("{:?}", client.databases);
    let created: bool = new_database2.create_database(&mut client).unwrap();
    println!("{:?}", created);
    println!("{:?}", client.databases);
    let created: bool = new_database3.create_database(&mut client).unwrap();
    println!("{:?}", created);
    println!("{:?}", client.databases);
    let db_info: Database = new_database1.read_database(&mut client).unwrap();
    println!("{:?}", db_info);
    let db_info: Database = new_database2.read_database(&mut client).unwrap();
    println!("{:?}", db_info);

    println!("{:?}", client.databases);

    let deleted: bool = new_database1.drop_database(&mut client).unwrap();
    println!("{:?}", deleted);
    new_database2.drop_database(&mut client).unwrap();
    new_database3.drop_database(&mut client).unwrap();

    println!("{:?}", client.databases);

    // let mut vect = Vec::new();
    // vect.push(1);
    // vect.push(2);
    // vect.push(3);
    // vect[1] = 4;
}
