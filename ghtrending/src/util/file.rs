use futures::prelude::*;
use serde_json::Value;
use std::time::SystemTime;
use tokio::fs;
use tokio_serde::{formats::SymmetricalJson, SymmetricallyFramed};
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

pub const CACHE_DEV_FILE: &str = "cache_dev.json";
pub const CACHE_REPO_FILE: &str = "cache_repo.json";

#[derive(Debug, Copy, Clone)]
pub enum FileName<'a> {
    CacheRepoFile(&'a str),
    CacheDevFile(&'a str),
}

pub async fn cache<'a>(
    data: Value,
    f: FileName<'_>,
) -> Result<(), Box<dyn std::error::Error + 'a>> {
    // Open a file, create if it doesn't exist
    let name = match f {
        FileName::CacheDevFile(file) => file,
        FileName::CacheRepoFile(file) => file,
    };

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(name)
        .await?;
    // Crate a length delimited codec
    let lenght_delimited = FramedWrite::new(&mut file, LengthDelimitedCodec::new());
    // Create a json codec
    let mut serialized =
        SymmetricallyFramed::new(lenght_delimited, SymmetricalJson::<Value>::default());
    serialized.send(data).await?;
    Ok(())
}

pub async fn load<'a>(f: FileName<'_>) -> Result<Value, Box<dyn std::error::Error + 'a>> {
    let name = match f {
        FileName::CacheDevFile(file) => file,
        FileName::CacheRepoFile(file) => file,
    };
    let file = fs::OpenOptions::new().read(true).open(name).await?;
    let metadata = file.metadata().await?;
    match metadata.modified() {
        Ok(modified) => {
            let elapsed = SystemTime::now()
                .duration_since(modified)
                .expect("Time went backwards.");
            if elapsed.as_secs() > 60 * 60 * 12 {
                Err("Data expired".into())
            } else {
                println!("cache hit!!!");
                // Delimit frames using a length header
                let length_delimited = FramedRead::new(file, LengthDelimitedCodec::new());
                // Deserialize frames
                let mut deserialized =
                    SymmetricallyFramed::new(length_delimited, SymmetricalJson::<Value>::default());

                if let Some(data) = deserialized.try_next().await.unwrap() {
                    Ok(data)
                } else {
                    Err("No data".into())
                }
            }
        }
        Err(err) => Err(err.into()),
    }
}
