local source = {}

local items = require("cmp_latex_symbols.items")

-- Julia's own LaTeX symbols generated using Julia's REPL built-in package
local extra_items = require("cmp_latex_symbols.extra_items")

for _, v in ipairs(extra_items) do
	table.insert(items, v)
end

-- Add another symbol
table.insert(items, {word="\\mathord", label="\\mathord ⍹", insertText="⍹", filterText="\\mathord"})

source.new = function()
	return setmetatable({}, { __index = source })
end

source.get_trigger_characters = function()
	return { "\\" }
end

source.get_keyword_pattern = function()
	return "\\\\[^[:blank:]]*"
end

source.complete = function(self, request, callback)
	if not vim.regex(self.get_keyword_pattern() .. "$"):match_str(request.context.cursor_before_line) then
		return callback()
	end
	if not self.items then
		self.items = items
	end
	callback(self.items)
end

return source
