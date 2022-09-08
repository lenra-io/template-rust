use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Default)]
pub struct Api {
    pub url: String,
    pub token: String,
}

impl Api {
    pub fn get_doc<T: Doc>(&self, coll: &str, id: u32) -> Result<T, Box<dyn std::error::Error>> {
        log::debug!("get_doc {}[{}]", coll, id);
        let request_url = format!("{url}/app/colls/{coll}/docs/{id}", url = self.url, id = id);

        ureq::get(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .call()?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn create_doc<T: Doc>(
        &self,
        coll: &str,
        doc: T,
    ) -> Result<T, Box<dyn std::error::Error>> {
        log::debug!("create_doc {}", serde_json::to_string(&doc).unwrap());

        let request_url = format!("{url}/app/colls/{coll}/docs", url = self.url);

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(doc)?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn update_doc<T: Doc>(
        &self,
        coll: &str,
        doc: T,
    ) -> Result<T, Box<dyn std::error::Error>> {
        log::debug!("update_doc {}", serde_json::to_string(&doc).unwrap());

        let request_url = format!(
            "{url}/app/colls/{coll}/docs/{id}",
            url = self.url,
            id = doc.id().unwrap()
        );

        ureq::put(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(doc)?
            .into_json()
            .map_err(|e| e.into())
    }

    pub fn delete_doc<T: Doc>(
        &self,
        coll: &str,
        doc: T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request_url = format!(
            "{url}/app/colls/{coll}/docs/{id}",
            url = self.url,
            id = doc.id().unwrap()
        );

        ureq::delete(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .call()?;

        Ok(())
    }

    pub fn execute_query<T: Doc, Q: Serialize>(
        &self,
        coll: &str,
        query: Q,
    ) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        log::debug!("execute_query {}", serde_json::to_string(&query).unwrap());
        let request_url = format!("{url}/app/colls/${coll}/docs/filter", url = self.url);

        ureq::post(request_url.as_str())
            .set("Authorization", format!("Bearer {}", self.token).as_str())
            .send_json(query)?
            .into_json()
            .map_err(|e| e.into())
    }
}

pub trait Doc: Sized + DeserializeOwned + Serialize + 'static + Clone {
    fn id(&self) -> Option<String>;
}
