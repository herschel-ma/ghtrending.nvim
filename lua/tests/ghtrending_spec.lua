local cfg = require('ghtrending.config')
local eq = assert.are.same

describe('ghtrending plugin', function()
  it('should load module', function() assert(cfg ~= nil) end)
  it('should read config', function() eq(cfg.chinese, true) end)
end)
