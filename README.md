## ghtrending.nvim

A plugin for neovim to view github trending repos.
BTW, written by Rust and Lua language.

## Prerequisites

### Required

- `neovim >= 0.5`
- Rust.(cargo inclusive)

## Installation

Example using `lazy.nvim`:

```lua
return {
  {
    "herschel-ma/ghtrending_nvim",
    dependencies = { "MunifTanjim/nui.nvim" },
    -- windows && nu
    build = "cargo build --release; mv target/release/ghtrending_nvim.dll  lua/ghtrending_nvim.dll",
    -- linux && bash(not test)
    -- build = "cargo build --release && mv target/release/ghtrending_nvim.so lua/ghtrending_nvim.so"
    -- macos && bash(not test)
    -- build = "cargo build --release && mv target/release/ghtrending_nvim.dylib lua/ghtrending_nvim.dll"
    config = function()
      require("ghtrending").setup()
    end,
  },
}
```

## Usage

### View Trending Repositories

A command `:GhtrendingRepo` present to popup a window to display github trending repos.

### View Trending Developers

A command `:GhtrendingDev` present to popup a window to display github trending developers.

## Default Key Mappings

Left pane:

```lua
popups.left_popup:map("n", "q", function()
  layout:unmount()
end, { silent = true })
popups.left_popup:map("n", "<esc>", function()
  layout:unmount()
end, { silent = true })
popups.left_popup:map("n", "L", function()
  vim.api.nvim_set_current_win(popups.right_popup.winid)
end, { silent = true })
```

Right pane:

```lua
popups.right_popup:map("n", "H", function()
  vim.api.nvim_set_current_win(popups.left_popup.winid)
end, { silent = true })
```
