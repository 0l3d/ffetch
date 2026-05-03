//go:build mage
// +build mage

package main

import (
	"os"
	"os/exec"
	"path/filepath"
	"runtime"
)

var Default = Build

func Build() error {
	_, file, _, ok := runtime.Caller(0)
	if !ok {
		return os.ErrInvalid
	}

	projectRoot := filepath.Dir(file)
	cmd := exec.Command("cargo", "build", "--locked")
	cmd.Dir = projectRoot
	cmd.Env = append(os.Environ(), "CARGO_TARGET_DIR="+filepath.Join(projectRoot, "target"))
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Stdin = os.Stdin

	return cmd.Run()
}
