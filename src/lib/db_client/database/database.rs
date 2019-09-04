use crate::db_client::base_response::base_response::BaseResponse;
use crate::db_client::db_client::DBClient;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// An arangoDb database rappresentation
pub struct Database {
    pub name: String,
    pub id: Option<String>,
    pub path: Option<String>,
    pub is_system: Option<bool>,
}

/// Trait implementation
impl PartialEq for Database {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Clone for Database {
    fn clone(&self) -> Database {
        match self.id {
            Some(_) => Database {
                name: (self.name.clone()),
                id: (self.id.clone()),
                path: (self.path.clone()),
                is_system: (self.is_system),
            },
            None => Database::new_local(&self.name),
        }
    }
}

impl Database {
    /// Creates a "local" db, this db is handled by rust itself
    /// only when the id is set the db can be considered created
    pub fn new_local(name: &str) -> Database {
        Database {
            name: String::from(name),
            id: None,
            path: None,
            is_system: None,
        }
    }

    /// This function asks for a Client instance to create the DB to
    /// the remote source, this will also append the Database instance to the db_client.databases vector
    pub fn create_database(&self, db_client: &mut DBClient) -> Result<bool, String> {
        let final_url: String = format!("{}{}", db_client.base_url, "/_api/database");
        match db_client.client.post(&final_url, self) {
            Ok(mut res) => {
                if res.status().is_success() {
                    db_client.databases.push(self.clone());
                    let result: BaseResponse<bool> = res.json().unwrap();
                    Ok(result.result)
                } else {
                    Err(res.text().unwrap())
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    /// Gets currently selected database informations, these will be inside the Database instance
    /// and will also be returned
    pub fn get_database_info(&mut self, db_client: &mut DBClient) -> Result<Database, String> {
        let final_url: String = format!(
            "{}/_db/{}{}",
            db_client.base_url, self.name, "/_api/database/current"
        );
        match db_client.client.get(&final_url) {
            Ok(mut res) => {
                if res.status().is_success() {
                    let result: BaseResponse<Database> = res.json().unwrap();
                    self.id = result.result.id.clone();
                    self.is_system = result.result.is_system;
                    self.path = result.result.path.clone();
                    // Find and replace the db in the client
                    let index = db_client
                        .databases
                        .iter()
                        .position(|db| db.name == self.name)
                        .unwrap();
                    db_client.databases[index] = self.clone();
                    Ok(result.result)
                } else {
                    Err(res.text().unwrap())
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    /// Delete a database
    pub fn drop_database(&mut self, db_client: &mut DBClient) -> Result<bool, String> {
        let final_url: String = format!("{}/_api/database/{}", db_client.base_url, self.name);
        match db_client.client.delete(&final_url) {
            Ok(mut res) => {
                if res.status().is_success() || res.status().eq(&404) {
                    db_client.databases.remove_item(&self);
                    Ok(true)
                } else {
                    Err(res.text().unwrap())
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }
}
