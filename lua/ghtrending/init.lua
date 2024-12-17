local M = {}
local display = {}

local gh = require("gh.rust")
local ns = vim.api.nvim_create_namespace("ghtrending")

local nui_text = require("nui.text")
local nui_popup = require("nui.popup")
local nui_layout = require("nui.layout")
local nui_table = require("nui.table")
local event = require("nui.utils.autocmd").event
local config = require("ghtrending.config")

local function clear_buffer(buf)
	vim.api.nvim_buf_set_option(buf.bufnr, "modifiable", true)
	vim.api.nvim_buf_set_lines(buf.bufnr, 0, -1, false, {})
	vim.api.nvim_buf_set_option(buf.bufnr, "modifiable", false)
	for _, extmark in ipairs(vim.api.nvim_buf_get_extmarks(buf.bufnr, ns, 0, -1, {})) do
		vim.api.nvim_buf_del_extmark(buf.bufnr, ns, extmark[1])
	end
end

---@param opts  table
local function fill_buffer(buf, opts)
	local datas = opts.datas
	local line = opts.line or nil
	local is_repo = opts.is_repo or nil

	clear_buffer(buf)
	local lines = {}
	if line ~= nil then
		for i, data in ipairs(datas) do
			if line == i then
				if is_repo == true then
					table.insert(lines, " " .. config.segment.author .. ":")
					table.insert(lines, "     " .. data.author)
					table.insert(lines, " " .. config.segment.description .. ":")
					table.insert(lines, "     " .. data.description)
					table.insert(lines, " " .. config.segment.star_count .. ":")
					table.insert(lines, "     " .. data.star_count)
					table.insert(lines, " " .. config.segment.add .. ":")
					table.insert(lines, "     " .. data.add)
					table.insert(lines, " " .. config.segment.forks .. ":")
					table.insert(lines, "     " .. data.forks)
					table.insert(lines, " " .. config.segment.language .. ":")
					table.insert(lines, "     " .. data.language)
					table.insert(lines, " " .. config.segment.build_by .. ":")
					for j, collaborator in ipairs(data.build_by) do
						table.insert(lines, "     " .. j .. "." .. collaborator.name)
						table.insert(lines, "         " .. collaborator.avatar)
					end
					table.insert(lines, " " .. config.segment.link .. ":")
					table.insert(lines, "     " .. data.link)

					vim.api.nvim_buf_set_option(buf.bufnr, "modifiable", true)
					vim.api.nvim_buf_set_lines(buf.bufnr, 0, -1, false, lines)
					vim.api.nvim_buf_set_option(buf.bufnr, "modifiable", false)
					-- data.build_by = nil -- set to none to avoid table render fail
					-- display:render_table(buf.bufnr, {
					-- 	datas = { data },
					-- 	is_repo = is_repo,
					-- })
				else
					display:render_table(buf.bufnr, { datas = { data } })
				end
			end
		end
	else
		for i, data in ipairs(datas) do
			table.insert(lines, " " .. i .. "." .. data.name)
			vim.api.nvim_buf_set_option(buf.bufnr, "modifiable", true)
			vim.api.nvim_buf_set_lines(buf.bufnr, 0, -1, false, lines)
			vim.api.nvim_buf_set_option(buf.bufnr, "modifiable", false)
		end
	end
end

--- @param datas object
--- @param opts table
function display:init(datas, opts)
	-- NUI elements(two popups)
	if opts.is_repo == true then
		self.span = config.segment.span_repo
	else
		self.span = config.segment.span_dev
	end

	local span_name = {
		bottom = nui_text(self.span),
		bottom_align = "center",
	}
	local popups = {
		left_popup = nui_popup({
			enter = true,
			border = {
				style = config.popup.border.style,
				text = span_name,
			},
			buf_options = {
				modifiable = false,
				readonly = true,
			},
			win_options = {
				winblend = config.popup.win_options.winblend,
				winhighlight = config.popup.win_options.winhighlight,
			},
			ns_id = ns,
		}),
		right_popup = nui_popup({
			focusable = true,
			border = "single",
			buf_options = {
				modifiable = true,
				readonly = false,
			},
			win_options = config.popup.win_options,
			ns_id = ns,
		}),
	}

	local layout = nui_layout(
		config.layout,
		nui_layout.Box({
			nui_layout.Box(popups.left_popup, { size = config.left_popup_size }),
			nui_layout.Box(popups.right_popup, { size = config.right_popup_size }),
		}, { dir = "row", grow = 1 })
	)

	vim.api.nvim_buf_set_option(popups.right_popup.bufnr, "filetype", "ghtrending")
	fill_buffer(popups.left_popup, { datas = datas, is_repo = opts.is_repo })
	-- Autocmds
	popups.left_popup:on(event.CursorMoved, function()
		local line, _ = unpack(vim.api.nvim_win_get_cursor(popups.left_popup.winid))
		fill_buffer(popups.right_popup, { datas = datas, line = line, is_repo = opts.is_repo })
	end)

	for _, p in pairs(popups) do
		p:on("BufLeave", function()
			vim.schedule(function()
				local bufnr = vim.api.nvim_get_current_buf()
				for _, lp in pairs(popups) do
					if lp.bufnr == bufnr then
						return
					end
				end
				layout:unmount()
			end)
		end)
	end

	-- Mapping
	popups.left_popup:map("n", "q", function()
		layout:unmount()
	end, { silent = true })
	popups.left_popup:map("n", "<esc>", function()
		layout:unmount()
	end, { silent = true })
	popups.left_popup:map("n", "L", function()
		vim.api.nvim_set_current_win(popups.right_popup.winid)
	end, { silent = true })

	popups.right_popup:map("n", "H", function()
		vim.api.nvim_set_current_win(popups.left_popup.winid)
	end, { silent = true })

	layout:mount()

	vim.api.nvim_win_set_option(popups.right_popup.winid, "signcolumn", "no")
	vim.api.nvim_win_set_option(popups.right_popup.winid, "foldlevel", 100)
	vim.api.nvim_win_set_option(popups.right_popup.winid, "wrap", false)
end
--- @param bufnr integer
--- @param opts table
function display:render_table(bufnr, opts)
	local is_repo = opts.is_repo or nil
	local datas = opts.datas
	local tbl = {}

	if is_repo == true then
		tbl = nui_table({
			ns_id = ns,
			bufnr = bufnr,
			columns = {
				{
					align = "center",
					accessor_key = "name",
					header = "Repository Name",
				},
				{
					align = "center",
					accessor_key = "author",
					header = "Author",
				},
				{
					align = "left",
					accessor_key = "description",
					header = "Description",
				},
				{
					align = "center",
					accessor_key = "star_count",
					header = "Star",
					cell = function(cell)
						return nui_text(tostring(cell.get_value()), "DiagnosticInfo")
					end,
				},
				{
					align = "left",
					accessor_key = "add",
					header = "Add Trending",
				},
				{
					align = "center",
					accessor_key = "fork",
					header = "Fork",
					cell = function(cell)
						return nui_text(tostring(cell.get_value()), "DiagnosticInfo")
					end,
				},
				{
					align = "center",
					accessor_key = "language",
					header = "Language",
				},
				{
					align = "left",
					accessor_key = "link",
					cell = function(cell)
						return nui_text(tostring(cell.get_value()), "DiagnosticInfo")
					end,
					header = "Repository url",
				},
			},
			data = datas,
		})
	else
		tbl = nui_table({
			ns_id = ns,
			bufnr = bufnr,
			columns = {
				{
					align = "center",
					accessor_key = "name",
					header = "Name",
				},
				{
					align = "left",
					accessor_key = "avatar",
					header = "Avatar",
				},
				{
					align = "left",
					accessor_key = "description",
					cell = function(cell)
						return nui_text(tostring(cell.get_value()), "DiagnosticInfo")
					end,
					header = "Description",
				},
				{
					align = "left",
					accessor_key = "popular_repo",
					cell = function(cell)
						return nui_text(tostring(cell.get_value()), "DiagnosticInfo")
					end,
					header = "Popular Repository",
				},
			},
			data = datas,
		})
	end

	tbl:render()
end

--- @param opts? table
M.setup = function(opts)
	opts = opts or {}
	for key, value in pairs(opts) do
		if config[key] ~= nil then
			config[key] = value
		end
	end
	-- create two custom functions for user to use
	vim.api.nvim_create_user_command("GhtrendingDev", function()
		local devlopers = gh.process_developer()
		-- local devlopers = {
		-- 	{
		-- 		name = "test1",
		-- 		avatar = "https://avatars.githubusercontent.com/u/10101010?v=4",
		-- 		popular_repo = "https://github.com/username/test1",
		-- 		description = "desc test1",
		-- 	},
		-- 	{
		-- 		name = "test2",
		-- 		avatar = "https://avatars.githubusercontent.com/u/10101010?v=5",
		-- 		popular_repo = "https://github.com/username/test2",
		-- 		description = "desc test2",
		-- 	},
		-- }
		M.devlopers = devlopers or nil
		if M.devlopers ~= nil then
			display:init(M.devlopers, { is_repo = false })
		else
			vim.notify(
				"GitHub Api Developers are nil! Run `:checkhealth ghtrending` for more info.",
				vim.log.levels.ERROR
			)
		end
	end, { bang = true })

	vim.api.nvim_create_user_command("GhtrendingRepo", function()
		local repos = gh.process_repo()
		M.repos = repos or nil
		if M.repos ~= nil then
			display:init(M.repos, { is_repo = true })
		else
			vim.notify(
				"GitHub Api Repositories are nil! Run `:checkhealth ghtrending` for more info.",
				vim.log.levels.ERROR
			)
		end
	end, { bang = true })
end

vim.api.nvim_create_user_command("GhtrendingOpenDev", function()
	local name = vim.api.nvim_get_current_line()
	local index
	if name ~= nil then
		local index_str
		for num in string.gmatch(name, "%d+") do
			index_str = num
		end
		index = tonumber(index_str)
	else
		index = nil
	end

	local q = M.devlopers[index]
	if q then
		local command
		local os_name = vim.loop.os_uname().sysname

		if os_name == "Linux" then
			command = string.format("xdg-open '%s'", q.popular_repo)
		elseif os_name == "Darwin" then
			command = string.format("open '%s'", q.popular_repo)
		else
			-- Fallback to Windows if uname is not available or does not match Linux/Darwin.
			command = string.format('start "" "%s"', q.popular_repo)
		end

		os.execute(command)
	end
end, { bang = true })

vim.api.nvim_create_user_command("GhtrendingOpenRepo", function()
	local name = vim.api.nvim_get_current_line()
	local index
	if name ~= nil then
		local index_str
		for num in string.gmatch(name, "%d+") do
			index_str = num
		end
		index = tonumber(index_str)
	else
		index = nil
	end

	local q = M.repos[index]
	if q then
		local command
		local os_name = vim.loop.os_uname().sysname

		if os_name == "Linux" then
			command = string.format("xdg-open '%s'", q.link)
		elseif os_name == "Darwin" then
			command = string.format("open '%s'", q.link)
		else
			command = string.format('start "" "%s"', q.link)
		end

		os.execute(command)
	end
end, { bang = true })

return M
