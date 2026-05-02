const std = @import("std");

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const host = b.graph.host;
    if (!targetsMatch(host.result, target.result)) {
        @panic("cross-compilation is not supported by this build.zig; use native target with cargo/rustc available in PATH");
    }

    const cargo_profile = profileForOptimize(optimize);
    const cargo_output_dir = outputDirForProfile(cargo_profile);

    const cargo_cmd = cargoBuildCommand(b, cargo_profile);
    const cargo_build = b.addSystemCommand(cargo_cmd);
    cargo_build.setEnvironmentVariable("CARGO_TARGET_DIR", b.pathFromRoot("target"));

    const install_bin = b.addInstallBinFile(
        b.path(b.pathJoin(&.{ cargo_output_dir, "ffetch" })),
        "ffetch",
    );
    install_bin.step.dependOn(&cargo_build.step);
    b.getInstallStep().dependOn(&install_bin.step);

    const run_cmd = b.addSystemCommand(&.{b.getInstallPath(.bin, "ffetch")});
    if (b.args) |args| run_cmd.addArgs(args);
    run_cmd.step.dependOn(b.getInstallStep());

    const run_step = b.step("run", "Run ffetch built via cargo");
    run_step.dependOn(&run_cmd.step);

    const check_cmd = b.addSystemCommand(&.{ "cargo", "check", "--locked" });
    check_cmd.setEnvironmentVariable("CARGO_TARGET_DIR", b.pathFromRoot("target"));
    const check_step = b.step("check", "Run cargo check");
    check_step.dependOn(&check_cmd.step);

    const test_cmd = b.addSystemCommand(&.{ "cargo", "test", "--locked" });
    test_cmd.setEnvironmentVariable("CARGO_TARGET_DIR", b.pathFromRoot("target"));
    const test_step = b.step("test", "Run cargo test");
    test_step.dependOn(&test_cmd.step);
}

fn targetsMatch(a: std.Target, b: std.Target) bool {
    return a.cpu.arch == b.cpu.arch and
        a.os.tag == b.os.tag and
        a.abi == b.abi;
}

fn profileForOptimize(optimize: std.builtin.OptimizeMode) []const u8 {
    return switch (optimize) {
        .Debug => "debug",
        .ReleaseSafe => "release",
        .ReleaseFast => "performance",
        .ReleaseSmall => "size",
    };
}

fn outputDirForProfile(profile: []const u8) []const u8 {
    if (std.mem.eql(u8, profile, "debug")) return "target/debug";
    if (std.mem.eql(u8, profile, "release")) return "target/release";
    if (std.mem.eql(u8, profile, "performance")) return "target/performance";
    if (std.mem.eql(u8, profile, "size")) return "target/size";
    @panic("unsupported cargo profile");
}

fn cargoBuildCommand(b: *std.Build, profile: []const u8) []const []const u8 {
    if (std.mem.eql(u8, profile, "debug")) {
        return b.allocator.dupe([]const u8, &.{ "cargo", "build", "--locked" }) catch @panic("OOM");
    }

    if (std.mem.eql(u8, profile, "release")) {
        return b.allocator.dupe([]const u8, &.{ "cargo", "build", "--release", "--locked" }) catch @panic("OOM");
    }

    return b.allocator.dupe([]const u8, &.{ "cargo", "build", "--profile", profile, "--locked" }) catch @panic("OOM");
}
