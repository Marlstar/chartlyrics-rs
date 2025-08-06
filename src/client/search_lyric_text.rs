use crate::api;
use crate::models::{SearchLyricResult, ArrayOfSearchLyricResult};
use crate::models::xml_to_model;

#[cfg(feature = "async")]
impl super::Client {
    pub async fn search_lyric_text(&self, text: &str) -> reqwest::Result<Vec<SearchLyricResult>> {
        let query = api::search_lyric_text(text);
        let response = self.query(&query).await?;
        let model = xml_to_model::<ArrayOfSearchLyricResult>(&response.text().await.unwrap()).unwrap();
        return Ok(model.0);
    }
}
#[cfg(feature = "blocking")]
impl super::BlockingClient {
    pub fn search_lyric_text(&self, text: &str) -> reqwest::Result<Vec<SearchLyricResult>> {
        let query = api::search_lyric_text(text);
        let response = self.query(&query)?;
        let model = xml_to_model::<ArrayOfSearchLyricResult>(&response.text().unwrap()).unwrap();
        return Ok(model.0);
    }
}
