#!/usr/bin/env node

declare const __dirname: string;
declare const process: {
  env: Record<string, string | undefined>;
  exit(code?: number): never;
};
declare function require(name: string): any;

const cp = require("node:child_process");
const path = require("node:path");

const projectRoot = __dirname;
const targetDir = path.join(projectRoot, "target");

const child = cp.spawnSync("cargo", ["build", "--locked"], {
  cwd: projectRoot,
  env: { ...process.env, CARGO_TARGET_DIR: targetDir },
  stdio: "inherit",
});

if (child.error) throw child.error;
if (child.status !== 0) process.exit(child.status || 1);
