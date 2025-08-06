# ChartLyrics
Rust bindings for the [ChartLyrics API](http://api.chartlyrics.com/apiv1.asmx).

# Features
- Get lyrics by song and artist (`search_lyric_direct`)
- Get lyrics by song id and checksum (`get_lyrics`)
- Find songs by lyrics (`search_lyric_text`)

# Crate Features
- Async (default; using the `async` feature)
- Blocking (using the `blocking` feature)

# Example
### Search Lyric Direct
```rust
// Async
use chartlyrics::Client;

#[tokio::main]
async fn main() {
    let client = Client::new().await.unwrap();
    let result = client.search_lyric_direct("Numb", "Linkin Park").await.unwrap();
    println!("{}", result.lyrics); // I'm tired of being what you want me to be...
}
```

```rust
// Blocking
use chartlyrics::BlockingClient;

fn main() {
    let client = BlockingClient::new().unwrap();
    let result = client.search_lyric_direct("Numb", "Linkin Park").unwrap();
    println!("{}", result.lyrics); // I'm tired of being what you want me to be...
}
```

### Find Song from Lyrics
```rust
// Async
use chartlyrics::Client;

#[tokio::main]
async fn main() {
    let client = Client::new().await.unwrap();
    let results = client.search_lyric_text("It starts with one thing").await.unwrap();
    for song in results {
        println!("{}", song.song); // Thriller, In the End, ...
    }
}
```

### Get Lyrics by ID
```rust
// Async
use chartlyrics::Client;

#[tokio::main]
async fn main() {
    let client = Client::new().await.unwrap();
    let result = client.get_lyrics(727, "d4fdd2eb33ad201aa860b52038298e05").await.unwrap();
    println!("{} by {}", result.song, result.artist); // In the End by Linkin Park
}
```
