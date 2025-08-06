# ChartLyrics
Rust bindings for the [ChartLyrics API](http://api.chartlyrics.com/apiv1.asmx).

# Features
- Get lyrics by song and artist (`search_lyric_direct`)

# Crate Features
- Async (default; using the `async` feature)
- Blocking (using the `blocking` feature)

# Example
```rust
// Async
use chartlyrics::Client;

#[tokio::main]
async fn main() {
    let client = Client::new().await.unwrap();
    let result = client.search_lyric_direct("Numb", "Linkin Park").await.unwrap();
    println!("{}", result.Lyric); // I'm tired of being what you want me to be...
}
```

```rust
// Blocking
use chartlyrics::BlockingClient;

#[tokio::main]
async fn main() {
    let client = BlockingClient::new().unwrap();
    let result = client.search_lyric_direct("Numb", "Linkin Park").unwrap();
    println!("{}", result.Lyric); // I'm tired of being what you want me to be...
}
```
