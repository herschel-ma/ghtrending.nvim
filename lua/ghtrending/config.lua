local M = {
  chinese = true,
  popup = {
    border = {
      style = 'single',
    },
    win_options = {
      winblend = 25,
      winhighlight = 'Normal:NormalFloat,FloatBorder:LineNr',
      scrolloff = 3,
      wrap = true,
    },
  },
  layout = {
    relative = 'editor',
    position = '50%',
    size = {
      width = '80%',
      height = '50%',
    },
  },
  left_popup_size = '30%',
  right_popup_size = '70%',
}

M.translate = function()
  if M.chinese then
    return {
      span_repo = '仓库名',
      span_dev = '开发者',
      name = '仓库',
      author = '作者',
      avatar = '头像',
      description = '描述',
      star_count = '星',
      add = '趋势',
      forks = '复刻',
      language = '语言',
      build_by = '协作者',
      link = '仓库地址',
    }
  else
    return {
      span_repo = 'Repos',
      span_dev = 'Developers',
      name = 'Name',
      author = 'Author',
      avatar = 'Avatar',
      description = 'Description',
      star_count = 'Star number',
      add = 'Add',
      forks = 'Forks',
      language = 'Language',
      build_by = 'Collaborator',
      link = 'Repository url',
    }
  end
end
M.segment = M.translate()
return M
