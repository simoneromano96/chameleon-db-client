use super::super::{BaseResponse, DBClient, Database};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
/// Collection model
pub struct Collection {
    pub id: Option<String>,
    pub name: String,
    pub status: Option<isize>,
    pub r#type: Option<isize>,
    pub is_system: Option<bool>,
    pub globally_unique_id: Option<String>,
    pub local_only: Option<bool>,
}

impl Clone for Collection {
    fn clone(&self) -> Collection {
        match self.id {
            Some(_) => Collection {
                name: self.name.clone(),
                id: self.id.clone(),
                status: self.status,
                r#type: self.r#type,
                is_system: self.is_system,
                globally_unique_id: self.globally_unique_id.clone(),
                local_only: self.local_only,
            },
            None => Collection::new_local(&self.name),
        }
    }
}

impl Collection {
    /// Creates a "local" collection, this collection is handled by rust itself
    /// only when the id is set the collection can be considered created
    pub fn new_local(name: &str) -> Collection {
        Collection {
            name: String::from(name),
            id: None,
            status: None,
            r#type: None,
            is_system: None,
            globally_unique_id: None,
            local_only: None,
        }
    }

    /// This function asks to the Client instance to create the Collection to
    /// the remote source, this will also append the Collection instance to the databases.collection vector
    pub fn create_collection(
        &self,
        db_client: &mut DBClient,
        database: &mut Database,
    ) -> Result<bool, String> {
        let final_url: String = format!(
            "{}/_db/{}{}",
            db_client.base_url, database.name, "/_api/collection"
        );

        match db_client.client.post(&final_url, self) {
            Ok(mut res) => {
                if res.status().is_success() {
                    database.collections.push(self.clone());
                    let result: BaseResponse<bool> = res.json().unwrap();
                    Ok(result.result)
                } else {
                    Err(res.text().unwrap())
                }
            }
            Err(err) => Err(err.to_string()),
        }
    }

    /// Gets currently selected collection informations, these will be inside the collection instance
    /// and will also be returned
    /// pub fn read_collection(
    ///     &mut self,
    ///     db_client: &mut DBClient,
    ///     database: &mut Database,
    /// ) -> Result<Collection, String> {
    ///     let final_url: String = format!(
    ///         "{}/_db/{}{}/{}",
    ///         db_client.base_url, database.name, "/_api/collection", self.name
    ///     );
    ///     match db_client.client.get(&final_url) {
    ///         Ok(mut res) => {
    ///             if res.status().is_success() {
    ///                 let result: BaseResponse<Collection>
    ///             }
    ///         }
    ///     }
    /// }
}
