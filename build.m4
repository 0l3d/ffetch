changequote([, ])dnl
define([BUILD_FFETCH], [
syscmd([CARGO_TARGET_DIR="$PWD/target" cargo build --locked])
ifelse(sysval, [0], [], [m4exit(sysval)])
])dnl
BUILD_FFETCH
