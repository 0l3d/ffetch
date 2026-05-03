#!/usr/bin/env lua

local separator = package.config:sub(1, 1)
local source = debug.getinfo(1, "S").source:gsub("^@", "")
local project_root = source:match("^(.*)[/\\][^/\\]+$") or "."

local function quote_posix(value)
  return "'" .. value:gsub("'", "'\\''") .. "'"
end

local function quote_windows(value)
  return '"' .. value:gsub('"', '""') .. '"'
end

local command
if separator == "\\" then
  command = "cd /d " .. quote_windows(project_root)
    .. " && set " .. quote_windows("CARGO_TARGET_DIR=" .. project_root .. "\\target")
    .. " && cargo build --locked"
else
  command = "cd " .. quote_posix(project_root)
    .. " && CARGO_TARGET_DIR=" .. quote_posix(project_root .. "/target")
    .. " cargo build --locked"
end

local ok, reason, code = os.execute(command)
if type(ok) == "number" then
  os.exit(ok)
end

if ok then
  os.exit(0)
end

if reason == "exit" and code then
  os.exit(code)
end

os.exit(1)
