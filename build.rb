#!/usr/bin/env ruby

require "English"

project_root = File.expand_path(__dir__)
ENV["CARGO_TARGET_DIR"] = File.join(project_root, "target")

Dir.chdir(project_root) do
  system("cargo", "build", "--locked")
  exit($CHILD_STATUS.exitstatus) unless $CHILD_STATUS.success?
end
