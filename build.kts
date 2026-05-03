#!/usr/bin/env kotlin

import java.io.File
import java.lang.ProcessBuilder
import kotlin.system.exitProcess

val projectRoot = File(System.getProperty("user.dir")).canonicalFile
val targetDir = File(projectRoot, "target").absolutePath

val process = ProcessBuilder("cargo", "build", "--locked")
    .directory(projectRoot)
    .inheritIO()
    .apply { environment()["CARGO_TARGET_DIR"] = targetDir }
    .start()

exitProcess(process.waitFor())
