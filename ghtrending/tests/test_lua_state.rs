use mlua::{chunk, Lua, Result};

#[cfg(test)]
mod tests {
    use super::*;
    use ghtrending::Repository;

    #[test]
    fn test_repository_state() -> Result<()> {
        // create a lua state
        let lua = Lua::new();
        let repo = Repository {
            name: "test".into(),
            ..Default::default()
        };
        lua.load(chunk! {
            local rep = $repo
            assert(rep.name == "test")
        })
        .exec()
    }
}

