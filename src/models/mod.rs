#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct GetLyricResult {
    #[serde(rename = "Lyric")] pub lyrics: String,
    TrackId: usize,
    LyricChecksum: Option<String>,
    #[serde(rename = "LyricId")] pub id: usize,
    #[serde(rename = "LyricSong")] pub song: String,
    #[serde(rename = "LyricArtist")] pub artist: String,
    LyricUrl: String,
    LyricCovertArtUrl: Option<String>,
    LyricRank: usize,
    LyricCorrectUrl: String,
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct SearchLyricResult {
    TrackId: usize,
    LyricChecksum: Option<String>,
    #[serde(rename = "LyricId")] pub id: usize,
    SongUrl: String,
    ArtistUrl: String,
    #[serde(rename = "Artist")] pub artist: String,
    #[serde(rename = "Song")] pub song: String,
    SongRank: String,
}
#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct ArrayOfSearchLyricResult(pub Vec<SearchLyricResult>);
impl std::ops::Deref for ArrayOfSearchLyricResult {
    type Target = Vec<SearchLyricResult>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

pub(crate) fn xml_to_model<T>(xml: &str) -> Result<T, serde_xml_rs::Error> where T: serde::de::DeserializeOwned {
    dbg!(xml);
    serde_xml_rs::from_str::<T>(xml)
}
