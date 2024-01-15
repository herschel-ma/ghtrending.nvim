use mlua::chunk;
use mlua::prelude::*;

fn hello(lua: &Lua, name: String) -> LuaResult<LuaTable> {
    let t = lua.create_table()?;
    t.set("name", name.clone())?;
    lua.load(chunk! {
        print("hello, " .. $name)
    })
    .exec()?;
    Ok(t)
}

#[mlua::lua_module]
fn ghtrending(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    let hello = lua.create_function(hello)?;
    exports.set("hello", hello)?;
    Ok(exports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ghtrending() -> LuaResult<()> {
        let lua = Lua::new();
        let t = hello(&lua, "world".into());
        dbg!(t.unwrap());
        Ok(())
    }
}
