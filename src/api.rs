use url_escape::encode_fragment as escape;

static API_URL: &str = "http://api.chartlyrics.com/apiv1.asmx";

pub fn search_lyric_direct(song: &str, artist: &str) -> String {
    let song = escape(song);
    let artist = escape(artist);
    format!("{API_URL}/SearchLyricDirect?song={song}&artist={artist}")
}

pub fn search_lyric_text(text: &str) -> String {
    let text = escape(text);
    format!("{API_URL}/SearchLyricText?lyricText={text}")
}
