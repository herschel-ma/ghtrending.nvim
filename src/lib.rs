use ghtrending::{process_devloper, process_repo, UserDataType};
use mlua::prelude::*;

fn process<F>(lua: &Lua, f: F) -> LuaResult<LuaTable>
where
    F: Fn() -> Result<Vec<UserDataType>, Box<dyn std::error::Error>>,
{
    let datas = f()
        .map_err(|e| mlua::Error::RuntimeError(format!("Error: {}", e)))
        .unwrap();
    let array_table = lua.create_table()?;

    for (i, data) in datas.into_iter().enumerate() {
        match data {
            UserDataType::Repository(repo) => array_table.set(i + 1, repo)?,
            UserDataType::Developer(dev) => array_table.set(i + 1, dev)?,
        };
    }
    Ok(array_table)
}

#[mlua::lua_module]
fn ghtrending_nvim(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let process_devloper = lua.create_function(|lua, ()| {
        process(lua, process_devloper).map_err(|err| mlua::Error::RuntimeError(err.to_string()))
    })?;
    let process_repo = lua.create_function(|lua, ()| {
        process(lua, process_repo).map_err(|err| mlua::Error::RuntimeError(err.to_string()))
    })?;

    exports.set("process_developer", process_devloper)?;
    exports.set("process_repo", process_repo)?;
    Ok(exports)
}
