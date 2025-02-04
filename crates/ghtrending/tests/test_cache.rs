use ghtrending::{cache, load, Developer, FileName, CACHE_DEV_FILE};

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cache() -> Result<(), Box<dyn std::error::Error>> {
        let mut devs = vec![];
        let f = FileName::CacheDevFile(CACHE_DEV_FILE);

        for i in 0..10 {
            let dev = Developer {
                name: format!("test_{}", i),
                avatar: format!("test_avatar_{}", i),
                description: format!("test_description_{}", i),
                popular_repo: format!("test_popular_repo_{}", i),
            };
            devs.push(dev)
        }

        let v = serde_json::to_value(&devs).unwrap();
        cache(v, f).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_load() -> Result<(), Box<dyn std::error::Error>> {
        let f = FileName::CacheDevFile(CACHE_DEV_FILE);
        let devs = load(f).await.unwrap();
        let deverlopers: Vec<Developer> = serde_json::from_value(devs).unwrap();
        assert_eq!(deverlopers.len(), 10);
        assert_eq!(deverlopers[0].name, "test_0");
        assert_eq!(deverlopers.last().unwrap().name, "test_9");
        Ok(())
    }
}
