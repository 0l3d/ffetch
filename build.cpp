#include <cstdlib>
#include <filesystem>
#include <iostream>
#include <string>
#include <system_error>

namespace fs = std::filesystem;

static bool has_cargo_toml(const fs::path& dir) {
    return fs::exists(dir / "Cargo.toml");
}

static bool find_repo_root_from(fs::path start, fs::path& out) {
    std::error_code ec;
    start = fs::absolute(start, ec);
    if (ec) return false;

    if (fs::is_regular_file(start, ec)) {
        start = start.parent_path();
    }

    for (fs::path dir = start; !dir.empty(); dir = dir.parent_path()) {
        if (has_cargo_toml(dir)) {
            out = dir;
            return true;
        }

        if (dir == dir.root_path()) break;
    }

    return false;
}

static bool find_repo_root(const char* argv0, fs::path& out) {
    if (find_repo_root_from(argv0, out)) return true;
    if (find_repo_root_from(fs::current_path(), out)) return true;
    if (find_repo_root_from(__FILE__, out)) return true;
    return false;
}

int main(int argc, char** argv) {
    (void)argc;

    fs::path project_root;
    if (!find_repo_root(argv[0], project_root)) {
        std::cerr << "failed to find repository root containing Cargo.toml\n";
        return 1;
    }

    const fs::path target_dir = project_root / "target";
#ifdef _WIN32
    _putenv_s("CARGO_TARGET_DIR", target_dir.string().c_str());
#else
    setenv("CARGO_TARGET_DIR", target_dir.string().c_str(), 1);
#endif

    std::error_code ec;
    fs::current_path(project_root, ec);
    if (ec) {
        std::cerr << "failed to change directory: " << ec.message() << "\n";
        return 1;
    }

    return std::system("cargo build --locked");
}
