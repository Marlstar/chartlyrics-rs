use crate::api;
use crate::models::GetLyricResult;
use crate::models::xml_to_model;

#[cfg(feature = "async")]
impl super::Client {
    pub async fn search_lyric_direct(&self, song: &str, artist: &str) -> reqwest::Result<GetLyricResult> {
        let query = api::search_lyric_direct(song, artist);
        let response = self.query(&query).await?;
        let model = xml_to_model::<GetLyricResult>(&response.text().await.unwrap()).unwrap();
        return Ok(model);
    }
}
#[cfg(feature = "blocking")]
impl super::BlockingClient {
    pub fn search_lyric_direct(&self, song: &str, artist: &str) -> reqwest::Result<GetLyricResult> {
        let query = api::search_lyric_direct(song, artist);
        let response = self.query(&query)?;
        let model = xml_to_model::<GetLyricResult>(&response.text().unwrap()).unwrap();
        return Ok(model);
    }
}
