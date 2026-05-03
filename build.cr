#!/usr/bin/env crystal

project_root = File.dirname(File.real_path(__FILE__))
target_dir = File.join(project_root, "target")

env = ENV.to_h
env["CARGO_TARGET_DIR"] = target_dir

status = Process.run(
  "cargo",
  ["build", "--locked"],
  chdir: project_root,
  env: env,
  input: Process::Redirect::Inherit,
  output: Process::Redirect::Inherit,
  error: Process::Redirect::Inherit
)

exit(status.exit_code)
