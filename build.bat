@echo off
setlocal

set "PROJECT_ROOT=%~dp0"
set "CARGO_TARGET_DIR=%PROJECT_ROOT%target"

pushd "%PROJECT_ROOT%" >nul || exit /b 1
cargo build --locked
set "STATUS=%ERRORLEVEL%"
popd >nul

exit /b %STATUS%
