local eq = assert.are.same
describe("ghtrending", function()
	it("require module", function()
		local M = require("ghtrending_nvim")
		assert(M ~= nil)
	end)
end)
