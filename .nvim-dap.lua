local dap = require("dap")

dap.adapters.lldb = {
	type = "executable",
	command = "/usr/bin/lldb-vscode-14",
	name = "lldb",
}

dap.configurations.rust = {
	{
		name = "lib-rdfa",
		type = "lldb",
		request = "launch",
		program = function()
			local test_binary = vim.fn.input("Path to test binary: ", vim.fn.getcwd() .. "/target/debug/", "file")
			return test_binary
		end,
		cwd = "${workspaceFolder}/lib-rdfa",
		stopOnEntry = false,
	},
}
