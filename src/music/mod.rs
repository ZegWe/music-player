use std::time::Duration;

#[derive(PartialEq)]
pub struct Music {
    name: String,
    artist: Option<String>,
    title: Option<String>,
    album: Option<String>,
    ext: Option<String>,
    lyric: Option<String>,
    duration: Duration,
}

impl Music {
}
