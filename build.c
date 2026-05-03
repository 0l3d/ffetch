#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <sys/wait.h>
#include <unistd.h>

static int has_cargo_toml(const char *dir) {
    char path[PATH_MAX];
    return snprintf(path, sizeof(path), "%s/Cargo.toml", dir) < (int)sizeof(path) &&
           access(path, F_OK) == 0;
}

static int find_repo_root_from(char *dir) {
    while (dir[0] != '\0') {
        if (has_cargo_toml(dir)) {
            return 1;
        }

        char *slash = strrchr(dir, '/');
        if (slash == NULL) {
            return 0;
        }

        if (slash == dir) {
            dir[1] = '\0';
            return has_cargo_toml(dir);
        }

        *slash = '\0';
    }

    return 0;
}

static int directory_from_path(const char *path, char *out, size_t out_len) {
    char resolved[PATH_MAX];
    if (realpath(path, resolved) == NULL) {
        return 0;
    }

    char *slash = strrchr(resolved, '/');
    if (slash == NULL) {
        return 0;
    }

    *slash = '\0';
    return snprintf(out, out_len, "%s", resolved) < (int)out_len;
}

static int find_repo_root(const char *argv0, char *out, size_t out_len) {
    char candidate[PATH_MAX];

    if (directory_from_path(argv0, candidate, sizeof(candidate)) && find_repo_root_from(candidate)) {
        return snprintf(out, out_len, "%s", candidate) < (int)out_len;
    }

    if (getcwd(candidate, sizeof(candidate)) != NULL && find_repo_root_from(candidate)) {
        return snprintf(out, out_len, "%s", candidate) < (int)out_len;
    }

    if (directory_from_path(__FILE__, candidate, sizeof(candidate)) && find_repo_root_from(candidate)) {
        return snprintf(out, out_len, "%s", candidate) < (int)out_len;
    }

    return 0;
}

int main(int argc, char **argv) {
    (void)argc;

    char project_root[PATH_MAX];
    char target_dir[PATH_MAX];

    if (!find_repo_root(argv[0], project_root, sizeof(project_root))) {
        fprintf(stderr, "failed to find repository root containing Cargo.toml\n");
        return 1;
    }

    if (snprintf(target_dir, sizeof(target_dir), "%s/target", project_root) >= (int)sizeof(target_dir)) {
        fprintf(stderr, "target path is too long\n");
        return 1;
    }

    if (setenv("CARGO_TARGET_DIR", target_dir, 1) != 0) {
        perror("setenv");
        return 1;
    }

    if (chdir(project_root) != 0) {
        perror("chdir");
        return 1;
    }

    int status = system("cargo build --locked");
    if (status == -1) {
        perror("system");
        return 1;
    }

    return WIFEXITED(status) ? WEXITSTATUS(status) : 1;
}
