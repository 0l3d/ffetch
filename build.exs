#!/usr/bin/env elixir

project_root = __DIR__
target_dir = Path.join(project_root, "target")

System.put_env("CARGO_TARGET_DIR", target_dir)

{_output, status} =
  System.cmd("cargo", ["build", "--locked"],
    cd: project_root,
    stderr_to_stdout: true,
    into: IO.stream(:stdio, :line)
  )

System.halt(status)
