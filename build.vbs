Option Explicit

Dim shell, fso, projectRoot, targetDir, status

Set shell = CreateObject("WScript.Shell")
Set fso = CreateObject("Scripting.FileSystemObject")

projectRoot = fso.GetParentFolderName(WScript.ScriptFullName)
targetDir = fso.BuildPath(projectRoot, "target")

shell.Environment("PROCESS")("CARGO_TARGET_DIR") = targetDir
shell.CurrentDirectory = projectRoot

status = shell.Run("cargo build --locked", 1, True)
WScript.Quit status
