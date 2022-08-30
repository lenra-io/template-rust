use serde_json::{from_value, json, Value};
// use reqwest::Error;
use serde::{de, Deserialize, Serialize};
use ureq::Error;

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Api {
    pub url: String,
    pub token: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct ApiResponse {
    pub data: ApiResponseData,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
struct ApiResponseData {
    pub id: Option<u32>,
    pub data: Value,
}

impl Api {
    pub fn get_data<T: Data>(&self, datastore: String, id: u32) -> Result<T, Error> {
        log::debug!("get_data {}[{}]", datastore, id);
        let request_url = format!(
            "{url}/app/datastores/{datastore}/data/{id}",
            url = self.url,
            datastore = datastore,
            id = id
        );

        let res = handle_response(
            datastore,
            ureq::get(request_url.as_str())
                .set("Authorization", format!("Bearer {}", self.token).as_str())
                .call()?
                .into_json()?,
        );

        log::debug!("/get_data {}", serde_json::to_string(&res).unwrap());

        Ok(res)
    }

    pub fn create_data<T: Data>(&self, data: T) -> Result<String, Error> {
        //Result<T, Error> {
        log::debug!("create_data {}", serde_json::to_string(&data).unwrap());
        let request_url = format!(
            "{url}/app/datastores/{datastore}/data",
            url = self.url,
            datastore = data.datastore().unwrap()
        );

        let res = ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(data)?
            .into_string()?;
        // .into_json()?;

        log::debug!("/create_data {}", serde_json::to_string(&res).unwrap());

        Ok(res)
    }

    pub fn update_data<T: Data>(&self, data: T) -> Result<T, Error> {
        log::debug!("update_data {}", serde_json::to_string(&data).unwrap());
        let datastore = data.datastore().unwrap();
        let request_url = format!(
            "{url}/app/datastores/{datastore}/data/{id}",
            url = self.url,
            datastore = datastore,
            id = data.id().unwrap()
        );

        let res = handle_response(
            datastore,
            ureq::put(request_url.as_str())
                .set("Authorization", format!("Bearer {}", self.token).as_str())
                .send_json(data)?
                .into_json()?,
        );

        log::debug!("/update_data {}", serde_json::to_string(&res).unwrap());

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
        log::debug!("create_datastore {}", datastore);
        let request_url = format!("{url}/app/datastores", url = self.url);

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(json!({ "name": datastore }))?
            .status();

        Ok(())
    }

    pub fn execute_query<T: Data, Q: Serialize>(&self, query: Q) -> Result<Vec<T>, Error> {
        log::debug!("execute_query {}", serde_json::to_string(&query).unwrap());
        let request_url = format!("{url}/app/query", url = self.url);

        let response: ApiResponse = ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(query)?
            .into_json()?;

        log::debug!(
            "/execute_query {}",
            serde_json::to_string(&response).unwrap()
        );

        Ok(from_value(response.data.data).unwrap())
    }
}

fn handle_response<T: Data>(datastore: String, response: ApiResponse) -> T {
    match response.data.data {
        Value::Object(mut data) => {
            data.insert("_datastore".into(), Value::String(datastore));
            if let Some(id) = response.data.id {
                data.insert("_id".into(), Value::Number(id.into()));
            }
            // let mut map = HashMap::new();
            // data.iter().for_each(|(key, value)| map.insert(key, value));
            from_value(Value::Object(data)).unwrap()
        }
        _ => panic!("Wrong response data type {}", response.data.data),
    }
}

pub trait Data: Sized + de::DeserializeOwned + Serialize + 'static + Clone {
    fn id(&self) -> Option<u32>;
    fn datastore(&self) -> Option<String>;
}
