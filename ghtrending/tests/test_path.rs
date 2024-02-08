use std::env;
use std::path::{Path, PathBuf};

#[cfg(test)]
mod tests {

    use super::*;
    fn project_root() -> PathBuf {
        Path::new(&env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .next()
            .unwrap()
            .to_path_buf()
    }

    #[test]
    fn test_current_dir() -> std::io::Result<()> {
        let path = env::current_dir()?;
        println!("The current directory is {}", path.display());

        Ok(())
    }

    #[test]
    fn test_current_exe() -> std::io::Result<()> {
        let path = env::current_exe()?;
        println!("The current exe dir is {}", path.display());

        Ok(())
    }

    #[test]
    fn test_project_root() {
        let path = project_root();
        println!("project path: {:?}", path.display())
    }
}
