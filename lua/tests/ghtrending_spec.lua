local eq = assert.are.same
describe("ghtrending", function()
	it("require plugin", function()
		require("ghtrending")
	end)

	it("greeting", function()
		local m = require("ghtrending")
		local grs = "greeting"
		eq(m.hello(grs).name, grs)
	end)
end)
