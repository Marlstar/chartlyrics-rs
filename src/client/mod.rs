mod search_lyric_direct;
mod search_lyric_text;
mod get_lyrics;

#[cfg(feature = "async")]
use reqwest::Client as AsyncReqwestClient;
#[cfg(feature = "blocking")]
use reqwest::blocking::Client as BlockingReqwestClient;

#[cfg(feature = "async")]
use reqwest::Response as AsyncResponse;
#[cfg(feature = "blocking")]
use reqwest::blocking::Response as BlockingResponse;

#[cfg(feature = "async")]
#[derive(Debug)]
pub struct Client {
    client: AsyncReqwestClient,
}
#[cfg(feature = "async")]
impl Client {
    pub async fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: AsyncReqwestClient::builder().build()?,
        })
    }

    async fn query(&self, query: &str) -> reqwest::Result<AsyncResponse> {
        self.client.execute(self.client.get(query).build()?).await
    }
}

#[cfg(feature = "blocking")]
#[derive(Debug)]
pub struct BlockingClient {
    client: BlockingReqwestClient,
}
#[cfg(feature = "blocking")]
impl BlockingClient {
    pub fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            client: BlockingReqwestClient::builder().build()?,
        })
    }

    fn query(&self, query: &str) -> reqwest::Result<BlockingResponse> {
        self.client.execute(self.client.get(query).build()?)
    }
}
