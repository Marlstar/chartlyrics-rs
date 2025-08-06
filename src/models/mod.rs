#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct GetLyricResult {
    pub Lyric: String,
    pub TrackId: usize,
    pub LyricChecksum: Option<String>,
    pub LyricId: usize,
    pub LyricSong: String,
    pub LyricArtist: String,
    pub LyricUrl: String,
    pub LyricCovertArtUrl: Option<String>,
    pub LyricRank: usize,
    pub LyricCorrectUrl: String,
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
#[derive(Debug, Clone)]
pub struct SearchLyricResult {
    pub TrackId: usize,
    pub LyricChecksum: Option<String>,
    pub LyricId: usize,
    pub SongUrl: String,
    pub ArtistUrl: String,
    pub Artist: String,
    pub Song: String,
    pub SongRank: String,
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
