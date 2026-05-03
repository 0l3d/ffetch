#!/usr/bin/env php
<?php
declare(strict_types=1);

$projectRoot = __DIR__;
$targetDir = $projectRoot . DIRECTORY_SEPARATOR . "target";

$command = "cargo build --locked";
$descriptorSpec = [
    0 => STDIN,
    1 => STDOUT,
    2 => STDERR,
];

$env = [];
foreach ([$_ENV, $_SERVER, ["CARGO_TARGET_DIR" => $targetDir]] as $source) {
    foreach ($source as $key => $value) {
        if (!is_string($key)) {
            continue;
        }

        if (is_scalar($value) || $value === null) {
            $env[$key] = (string) ($value ?? "");
        }
    }
}

$process = proc_open($command, $descriptorSpec, $pipes, $projectRoot, $env);
if (!is_resource($process)) {
    throw new RuntimeException("failed to start process");
}

$status = proc_close($process);
if ($status !== 0) {
    exit($status);
}

