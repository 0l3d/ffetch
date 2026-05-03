#!/usr/bin/env dotnet fsi

open System
open System.Diagnostics
open System.IO

let projectRoot = __SOURCE_DIRECTORY__
let targetDir = Path.Combine(projectRoot, "target")

let startInfo = ProcessStartInfo("cargo", "build --locked")
startInfo.WorkingDirectory <- projectRoot
startInfo.UseShellExecute <- false
startInfo.Environment["CARGO_TARGET_DIR"] <- targetDir

use process =
    Process.Start(startInfo)
    |> Option.ofObj
    |> Option.defaultWith (fun () -> invalidOp "failed to start cargo")

process.WaitForExit()
exit process.ExitCode
