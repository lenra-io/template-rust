use reqwest::Error;
use serde::{de, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Api {
    pub url: String,
    pub token: String,
}

impl Api {
    pub async fn get_data<T: Data>(&self, datastore: String, id: u32) -> Result<T, Error> {
        let client = reqwest::Client::new();
        let request_url = format!(
            "{url}/app/datastores/{datastore}/data/{id}",
            url = self.url,
            datastore = datastore,
            id = id
        );
        let response = client
            .get(&request_url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;
        response.json().await
    }

    pub async fn update_data<T: Data>(&self, data: T) -> Result<T, Error> {
        let client = reqwest::Client::new();
        let request_url = format!(
            "{url}/app/datastores/{datastore}/data/{id}",
            url = self.url,
            datastore = data.datastore(),
            id = data.id()
        );
        let response = client
            .put(&request_url)
            .header("Authorization", format!("Bearer {}", self.token))
            .send()
            .await?;
        response.json().await
    }
}

pub trait Data: de::DeserializeOwned {
    fn id(&self) -> u32;
    fn datastore(&self) -> String;
}
