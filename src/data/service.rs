use serde_json::json;
// use reqwest::Error;
use serde::{de, Deserialize, Serialize};
use ureq::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Api {
    pub url: String,
    pub token: String,
}

impl Api {
    pub fn get_data<T: Data>(&self, datastore: String, id: u32) -> Result<T, Error> {
        let request_url = format!(
            "{url}/app/datastores/{datastore}/data/{id}",
            url = self.url,
            datastore = datastore,
            id = id
        );

        let res = ureq::get(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .call()?
            .into_json()?;
        Ok(res)
    }

    pub fn create_data<T: Data>(&self, data: T) -> Result<T, Error> {
        log::info!("create_data {}", serde_json::to_string(&data).unwrap());
        let request_url = format!(
            "{url}/app/datastores/{datastore}/data",
            url = self.url,
            datastore = data.datastore().unwrap()
        );

        let res = ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(data)?
            .into_json()?;

        Ok(res)
    }

    pub fn update_data<T: Data>(&self, data: T) -> Result<T, Error> {
        log::info!("update_data {}", serde_json::to_string(&data).unwrap());
        let request_url = format!(
            "{url}/app/datastores/{datastore}/data/{id}",
            url = self.url,
            datastore = data.datastore().unwrap(),
            id = data.id().unwrap()
        );

        let res = ureq::put(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(data)?
            .into_json()?;

        Ok(res)
    }

    pub fn delete_data<T: Data>(&self, data: T) -> Result<(), Error> {
        let request_url = format!(
            "{url}/app/datastores/{datastore}/data/{id}",
            url = self.url,
            datastore = data.datastore().unwrap(),
            id = data.id().unwrap()
        );

        ureq::delete(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .call()?;

        Ok(())
    }

    pub fn create_datastore(&self, datastore: &str) -> Result<(), Error> {
        let request_url = format!("{url}/app/datastores", url = self.url);

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(json!({ "name": datastore }))?
            .status();

        Ok(())
    }

    pub fn execute_query<T: Data, Q: Serialize>(&self, query: Q) -> Result<T, Error> {
        let request_url = format!("{url}/app/query", url = self.url);

        let res = ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(query)?
            .into_json()?;

        Ok(res)
    }
}

pub trait Data: Sized + de::DeserializeOwned + Serialize {
    fn id(&self) -> Option<u32>;
    fn datastore(&self) -> Option<String>;
}
