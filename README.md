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
    "herschel-ma/ghtrending.nvim",
    -- dev = true,
    dependencies = { "MunifTanjim/nui.nvim" },
    build = "cargo build --release",
    config = function()
      require("ghtrending").setup()
      vim.keymap.set(
        { "n", "v" },
        "<leader>gtr",
        "<cmd>GhtrendingRepo<cr>",
        { silent = true, desc = "View Trednging Repos." }
      )
      vim.keymap.set(
        { "n", "v" },
        "<leader>gtd",
        "<cmd>GhtrendingDev<cr>",
        { silent = true, desc = "View Trending Developers" }
      )
      vim.api.nvim_set_keymap("n", "<leader>gtod", "<cmd>GhtrendingOpenDev<cr>", { silent = true, noremap = true })
      vim.api.nvim_set_keymap("n", "<leader>gtor", "<cmd>GhtrendingOpenRepo<cr>", { silent = true, noremap = true })
    end,
  },
}
```

## Configuration

Default Configuration:

```lua
local M = {
  chinese = true, -- default true, if false, show English
  popup = {
   border = {
      style = "single",  -- popup border style
     },
    win_options = {
      winblend = 25,
      winhighlight = "Normal:NormalFloat,FloatBorder:LineNr",
      scrolloff = 3,
      wrap = true,
    },
  },
  layout = {
    relative = "editor",
    position = "50%",
    size = {
      width = "80%",
      height = "50%",
    },
  },
  left_popup_size = "30%",
  right_popup_size = "70%",
}

-- require("ghtrending").setup(M) -- set config
```

> refer to [nui.nvim/lua/nui/popup](https://github.com/MunifTanjim/nui.nvim/tree/main/lua/nui/popup) for more detail.
> refer to [nui.nvim/lua/nui/layout](https://github.com/MunifTanjim/nui.nvim/tree/main/lua/nui/layout) for more detail.

## Usage

### View Trending Repositories

A command `:GhtrendingRepo` present to popup a window to display github trending repos.

### Open the repository under current cursor

A command `:GhtrendingOpenRepo` present to open the repository under current cursor with your default web browser.

### View Trending Developers

A command `:GhtrendingDev` present to popup a window to display github trending developers.

### Open the developer under current cursor

A command `:GhtrendingOpenDev` present to open the most popular repository of the developer under current cursor with your default web browser.

## Default Key Mappings on popup windows

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
popups.left_popup:map(
  'n',
  '<cr>',
  function() vim.api.nvim_set_current_win(popups.right_popup.winid) end,
  { silent = true }
)
```

Right pane:

```lua
popups.right_popup:map('n', 'q', function() layout:unmount() end, { silent = true })
popups.right_popup:map('n', '<esc>', function() layout:unmount() end, { silent = true })
popups.right_popup:map("n", "H", function()
  vim.api.nvim_set_current_win(popups.left_popup.winid)
end, { silent = true })
```
