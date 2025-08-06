use crate::api;
use crate::models::GetLyricResult;
use crate::models::xml_to_model;

#[cfg(feature = "async")]
impl super::Client {
    pub async fn get_lyrics(&self, id: usize, checksum: &str) -> reqwest::Result<GetLyricResult> {
        let query = api::get_lyric(id, checksum);
        let response = self.query(&query).await?;
        let model = xml_to_model::<GetLyricResult>(&response.text().await.unwrap()).unwrap();
        return Ok(model);
    }
}
#[cfg(feature = "blocking")]
impl super::BlockingClient {
    pub fn get_lyrics(&self, id: usize, checksum: &str) -> reqwest::Result<GetLyricResult> {
        let query = api::get_lyric(id, checksum);
        let response = self.query(&query)?;
        let model = xml_to_model::<GetLyricResult>(&response.text().unwrap()).unwrap();
        return Ok(model);
    }
}
