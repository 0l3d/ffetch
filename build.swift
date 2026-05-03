#!/usr/bin/env swift

import Foundation

let currentDirectory = URL(fileURLWithPath: FileManager.default.currentDirectoryPath)
let scriptURL = URL(fileURLWithPath: CommandLine.arguments[0], relativeTo: currentDirectory)
let projectRoot = scriptURL.standardized.deletingLastPathComponent()
let targetDir = projectRoot.appendingPathComponent("target").path

let process = Process()
process.executableURL = URL(fileURLWithPath: "/usr/bin/env")
process.arguments = ["cargo", "build", "--locked"]
process.currentDirectoryURL = projectRoot
process.environment = ProcessInfo.processInfo.environment.merging(
    ["CARGO_TARGET_DIR": targetDir],
    uniquingKeysWith: { _, new in new }
)

try process.run()
process.waitUntilExit()
exit(process.terminationStatus)
